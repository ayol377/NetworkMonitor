//!
//! 

use std::{net::Ipv4Addr, env};

use rusqlite::{self, Connection};
use tauri::api::path::data_dir;

use crate::structs::Device;

pub fn add_device(dev: Device){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    match Connection::open(path) {
        Ok(conn) => {

            let query = format!("SELECT mac FROM devices WHERE mac LIKE '{}'",
            dev.mac());
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let mut q_result = query.query_map([], |row| {
                Ok(Device::new(
                    row.get(0).unwrap(),
                    Ipv4Addr::new(1, 1, 1, 1),
                    "None".to_string(),
                    "None".to_string()
                ))
            }).unwrap();
            match q_result.next(){
                Some(_) => {
                    println!("Entry Exists!");
                    return;
                },
                None => {
                    let query = format!("INSERT INTO devices (mac, ip_add, manufacturer) VALUES ('{}', '{}', '{}')",
                    dev.mac(), dev.ip(), dev.manufacturer());
                    conn.execute(query.as_str(), ()).unwrap();
                },
            }
            
        },
        Err(_) => println!("Error opening DB!"),
    }
}

pub fn get_devices() -> Vec<Device>{
    let all_devs:Vec<Device> = vec![];



    return all_devs;
}