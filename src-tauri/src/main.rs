//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Imports
use std::{fs::{File, create_dir}, net::{Ipv4Addr, IpAddr}};
use if_addrs;
use system_info::network::Ip;

// Modules
mod bootstrapper;
mod database;
mod net_analyzer;
mod structs;

fn main() {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    println!("{}", path.display());
    match File::open(path) {
        Ok(_db) => {
            println!("Database Available")
        }
        Err(_) => bootstrapper::initialize_db(),
    }

    let mut dev_ip:IpAddr = IpAddr::V4(Ipv4Addr::new(1,1,1,1));
    for iface in if_addrs::get_if_addrs().unwrap(){
      if iface.is_link_local(){break;}
      if iface.is_loopback(){break;}
      if iface.addr.ip().is_ipv4() {
        dev_ip = iface.addr.ip();
        println!("My IP: {}", dev_ip);
      }
    }
    let mut dev_ipv4:Ipv4Addr = Ipv4Addr::new(1, 1 , 1, 1);
    match dev_ip {
        IpAddr::V4(ip) => dev_ipv4 = ip,
        IpAddr::V6(_) => {
          println!("Something went wrong");
        },
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hosts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn hosts() {}
