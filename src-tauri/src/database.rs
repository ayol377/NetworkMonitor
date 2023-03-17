//!
//!

use std::net::Ipv4Addr;

use rusqlite::{self, Connection};

use crate::structs::Device;

pub fn add_device(dev: Device) {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT mac FROM devices WHERE mac LIKE '{}'", dev.mac());
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let mut q_result = query
                .query_map([], |row| {
                    Ok(Device::new(
                        row.get(0).unwrap(),
                        Ipv4Addr::new(1, 1, 1, 1),
                        "None".to_string(),
                        "None".to_string(),
                    ))
                })
                .unwrap();
            match q_result.next() {
                Some(_) => {
                    println!("Entry Exists!");
                    return;
                }
                None => {
                    let query = format!(
                        "INSERT INTO devices (mac, ip_add, manufacturer) VALUES ('{}', '{}', '{}')",
                        dev.mac(),
                        dev.ip(),
                        dev.manufacturer()
                    );
                    conn.execute(query.as_str(), ()).unwrap();
                }
            }
        }
        Err(_) => println!("Error opening DB!"),
    }
}

pub fn get_devices() -> Vec<String> {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    let mut all_devs: Vec<String> = vec![];
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT mac FROM devices");
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let q_result = query
                .query_map([], |row| {
                    Ok(row.get(0).unwrap())
                })
                .unwrap();
            for mac in q_result {
                all_devs.push(mac.unwrap());
            }
        }
        Err(_) => println!("Error opening DB!"),
    }
    return all_devs;
}
