//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Imports
use crate::net_analyzer::*;
use std::{
    fs::{File},
    net::{IpAddr, Ipv4Addr},
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

    let app = tauri::Builder::default()
        // .setup(|app| {
        //     todo!();
        // })
        .invoke_handler(tauri::generate_handler![getnetwork])
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
    match net_analyzer::scan(dev_ipv4) {
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
