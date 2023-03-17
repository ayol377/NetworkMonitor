//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use database::get_devices;

// Imports
use crate::net_analyzer::*;
use std::{
    fs::{File},
    net::{IpAddr, Ipv4Addr}, process::Command,
};

// Modules
mod bootstrapper;
mod database;
mod net_analyzer;
mod structs;

fn main() {
    // Make sure database is available
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    match File::open(path) {
        Ok(_db) => {
            println!("Database Available")
        }
        Err(_) => bootstrapper::initialize_db(),
    }

    arpscan();
    pingnet();

    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![getnetwork])
        .invoke_handler(tauri::generate_handler![getdevs, getnetwork])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}

pub fn arpscan() {
    let dev_ip = getip();
    println!("My IP: {}", dev_ip);

    let mut dev_ipv4: Ipv4Addr = Ipv4Addr::new(1, 1, 1, 1);
    match dev_ip {
        IpAddr::V4(ip) => dev_ipv4 = ip,
        IpAddr::V6(_) => {
            println!("Something went wrong");
        }
    }
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
    println!("Function Called");
    let ip = getip();
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    let ip = ipnetwork::IpNetwork::network(&net);
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    return net.to_string();
}

#[tauri::command]
fn getdevs() -> Vec<String> {
    let devs = get_devices();
    for mac in &devs {
        println!("{}", mac);
    }
    return devs;
}

fn pingnet() {
    let net = getnet();
    let ip = net.broadcast();
    let mut cmd = Command::new("ping");
    cmd.arg(format!("{}", ip));
    match cmd.status() {
        Ok(_) => println!("OK"),
        Err(_) => println!("Err"),
    }
}
