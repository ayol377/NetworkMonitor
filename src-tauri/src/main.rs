//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::{get_devices, get_alerts};
use firebase::{create_user, signin};
use crate::net_analyzer::*;
use std::{
    fs::{*, self},
    net::Ipv4Addr, vec,
};
use tokio::{self};
use tauri::{SystemTray, SystemTrayEvent, Manager};
use tauri::{CustomMenuItem, SystemTrayMenu};

mod bootstrapper;
mod database;
mod net_analyzer;
mod structs;
mod security;
mod firebase;

static mut UP_DEVS: Vec<Ipv4Addr> = vec![];

#[tokio::main]
async fn main() {
    // Get Settings file. If unavailable bootstrap the app
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    match File::open(&path) {
        Ok(_) => {
           println!("Starting App"); 
        },
        Err(_) => bootstrapper::strap(),
    }

    tokio::task::spawn(async {pingscan(120).await});
    // tokio::task::spawn(async {security_coroutine(30, dns, etv, mitm).await});

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default().on_window_event(|event| match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
          event.window().hide().unwrap();
          api.prevent_close();
        }
        _ => {}
    })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                  app.exit(1);
                }
                _ => {}
              }
            }

            SystemTrayEvent::DoubleClick { .. } => {
                let w = app.get_window("main").unwrap();
                w.show().unwrap();
            }

            _ => {}
          })
        .invoke_handler(tauri::generate_handler![
            getdevs, getnetwork, getdev, get_settings,
            update_setting, get_alert_list, mapdata,
            firebase::logout, signup, getaccount, login,

        ])
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
async fn signup(email: String, passwd: String) -> String{
    let res = create_user(email, passwd).await.unwrap();
    return res;
}

#[tauri::command]
async fn login(email: String, passwd: String) -> String{
    let res = signin(email, passwd).await.unwrap();
    return res;
}

#[tauri::command]
fn getaccount() -> String{
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    let data = fs::read_to_string(&path).unwrap();
    let data = data.as_str();
    let settings = json::parse(data).unwrap();
    let email = settings["email"].as_str().unwrap();
    return email.to_string();
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
fn mapdata() -> Vec<Vec<String>> {
    let devs = get_devices();
    let mut return_devs: Vec<Vec<String>> = vec![];
    for dev in &devs {
        let mut newdev: Vec<String> = vec![];
        newdev.push(dev.hostname().to_string());
        newdev.push(dev.mac().to_string());
        newdev.push(dev.ip().to_string());

        unsafe { 
            let r_up_devs = UP_DEVS.clone();
            if r_up_devs.contains(&dev.ip()){
                newdev.push("up".to_string());
                return_devs.push(newdev);
            }else{
                newdev.push("down".to_string());
                return_devs.push(newdev);
            }
        }    

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
    let settings = json::parse(data).unwrap();

    if settings["dns"] == true {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if settings["mitm"] == true {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if settings["eviltwin"] == true {
        sett_string.push("1".to_string());
    }else {
        sett_string.push("0".to_string());
    }
    if settings["cloudbackup"] == true {
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
    let mut settings = json::parse(data).unwrap();
    let setting = setting.as_str();

    match setting {
        "dns" => {
            if settings["dns"] == true {
                settings["dns"] = false.into();
            }else{
                settings["dns"] = true.into();
            }
        },
        "mitm" => {
            if settings["mitm"] == true {
                settings["mitm"] = false.into();
            }else{
                settings["mitm"] = true.into();
            }
        },
        "eviltwin" => {
            if settings["eviltwin"] == true {
                settings["eviltwin"] = false.into();
            }else{
                settings["eviltwin"] = true.into();
            }
        },
        "cloudbackup" => {
            if settings["cloudbackup"] == true {
                settings["cloudbackup"] = false.into();
            }else{
                settings["cloudbackup"] = true.into();
            }
        },
        &_ =>{
            println!("Something whent wrong!");
        }
    }
    fs::write(path, json::stringify_pretty(settings, 1)).unwrap();
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