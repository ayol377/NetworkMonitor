//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::{get_devices, dev_state, get_alerts};
use crate::net_analyzer::*;
use std::{
    fs::{*, self},
    sync::{Arc, Mutex},
    net::{IpAddr, Ipv4Addr}, process::Command, time::{self, Duration}, task::Poll, fmt::format, ptr::eq, vec,
};
use tokio::{self};
use serde::{Deserialize, Serialize};

mod bootstrapper;
mod database;
mod net_analyzer;
mod structs;
mod security;


#[derive(Deserialize)]
#[derive(Serialize)]
struct settings{
    pub dns: bool,
    pub mitm: bool,
    pub eviltwin: bool,
    pub cloudbackup: bool
}

static mut UP_DEVS: Vec<Ipv4Addr> = vec![];

#[tokio::main]
async fn main() {

    let mut settings_v:Vec<String> = vec![];

    // Get Settings file. If unavailable bootstrap the app
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    match File::open(&path) {
        Ok(_) => {
           println!("OK!"); 
        },
        Err(_) => bootstrapper::strap(),
    }
    // bootstrapper::strap();

    unsafe {let pusher = |a| UP_DEVS.push(a);}
    unsafe{ tokio::task::spawn(async {pingscan(30).await}); }
    // tokio::task::spawn(async {security_coroutine(30, dns, etv, mitm).await});
    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![getnetwork])
        .invoke_handler(tauri::generate_handler![getdevs, getnetwork, getdev, get_settings, update_setting, get_alert_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}

pub fn arpscan() {
    let dev_ip = getip();
    println!("My IP: {}", dev_ip);
    match net_analyzer::scan() {
        Ok(devices) => {
            for device in devices {
                println!("Adding device to database");
                database::add_device(device);
            }
        }
        Err(_) => todo!(),
    }
}

#[tauri::command]
fn getnetwork() -> String {
    let ip = getip();
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    let ip = ipnetwork::IpNetwork::network(&net);
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    return net.to_string();
}

#[tauri::command]
fn getdevs() -> Vec<Vec<String>> {
    let devs = get_devices();
    let mut return_devs: Vec<Vec<String>> = vec![];
    let mut on_hosts: Vec<Vec<String>> = vec![];
    let mut of_hosts: Vec<Vec<String>> = vec![];
    for dev in &devs {
        let mut newdev: Vec<String> = vec![];
        newdev.push(dev.mac().to_string());
        newdev.push(dev.ip().to_string());
        newdev.push(dev.hostname().to_string());

        unsafe { 
            let r_up_devs = UP_DEVS.clone();
            if r_up_devs.contains(&dev.ip()){
                newdev.push("up".to_string());
                on_hosts.push(newdev);
            }else{
                newdev.push("down".to_string());
                of_hosts.push(newdev);
            }
        }    
        // if dev_state(dev.ip()){
        //     newdev.push("up".to_string());
        //     on_hosts.push(newdev);
        // }else{
        //     newdev.push("down".to_string());
        //     of_hosts.push(newdev);
        // }
    }
    for dev in on_hosts{
        return_devs.push(dev);
    }
    for dev in of_hosts{
        return_devs.push(dev);
    }
    return return_devs;
}

#[tauri::command]
fn getdev(mac: String) -> Vec<String> {
    let mut device:Vec<String> = vec![];
    let devs = get_devices();
    for dev in devs{
        if dev.mac() == mac{
            device.push(dev.hostname().to_string());
            device.push(dev.ip().to_string());
            device.push(dev.mac().to_string());
            device.push(dev.manufacturer().to_string());
        }
    }
    return device;
}

#[tauri::command(rename_all = "snake_case")]
fn get_settings() -> Vec<String> {
    let mut sett_string = vec![];
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    let data = fs::read_to_string(&path).unwrap();
    let data = data.as_str();
    let sett: settings = serde_json::from_str(data).unwrap();
    if sett.dns {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if sett.mitm {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if sett.eviltwin {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if sett.cloudbackup {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }

    return sett_string;
}

#[tauri::command(rename_all = "snake_case")]
fn update_setting(setting: String){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    let data = fs::read_to_string(&path).unwrap();
    let data = data.as_str();
    let mut sett: settings = serde_json::from_str(data).unwrap();
    let setting = setting.as_str();

    match setting {
        "dns" => {
            if sett.dns {
                sett.dns = false;
            }else{
                sett.dns = true;
            }
        },
        "mitm" => {
            if sett.mitm {
                sett.mitm = false;
            }else{
                sett.mitm = true;
            }
        },
        "eviltwin" => {
            if sett.eviltwin {
                sett.eviltwin = false;
            }else{
                sett.eviltwin = true;
            }
        },
        "cloudbackup" => {
            if sett.cloudbackup {
                sett.cloudbackup = false;
            }else{
                sett.cloudbackup = true;
            }
        },
        &_ =>{
            println!("Something whent wrong!");
        }
    }
    let conf = serde_json::to_string_pretty(&sett).unwrap();
    fs::write(path, conf).unwrap();
}

pub struct Alert{
    pub time: String,
    pub date: String,
    pub level: String,
    pub desc: String,
}

#[tauri::command(rename_all = "snake_case")]
fn get_alert_list() -> Vec<Vec<String>> {
    let mut alerts_v = vec![];
    let alerts = get_alerts();
    for alert in alerts{
        let t:Vec<String> = vec![alert.time, alert.date, alert.level, alert.desc];
        alerts_v.push(t);
    }
    return alerts_v;
}