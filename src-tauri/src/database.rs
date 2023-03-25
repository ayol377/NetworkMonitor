//!
//!

use std::net::Ipv4Addr;

use rusqlite::{self, Connection};

use crate::{structs::Device, net_analyzer::str_to_ip};

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
                    Ok(Device {
                        mac: row.get(0).unwrap(),
                        ip: Ipv4Addr::new(1, 1, 1, 1),
                        manufacturer: "UNKNOWN".to_string(),
                        joindate: "UNKNOWN".to_string(),
                    })
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

pub fn get_devices() -> Vec<Device> {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    let mut all_devs: Vec<Device> = vec![];
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT * FROM devices");
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let q_result = query
                .query_map([], |row| {
                    Ok(Device{
                        mac: row.get(0).unwrap(),
                        ip: str_to_ip(row.get(1).unwrap()),
                        manufacturer: "NULL".to_string(),
                        joindate: "NULL".to_string(),
                    })
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

pub fn mf_lookup (mac: String){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
}

pub fn is_up (state: bool, ip: Ipv4Addr){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("device_states.db");
    match Connection::open(path) {
        Ok(conn) => {
            if state {
                let query = format!("UPDATE devices SET state = 'up' WHERE ip_add = '{}'", ip);
                conn.execute(&query, ()).unwrap();
            }else{
                let query = format!("UPDATE devices SET state = 'down' WHERE ip_add = '{}'", ip);
                conn.execute(&query, ()).unwrap();
            }
        }
        Err(_) => println!("Error opening DB!"),
    }
}

struct devstate {
    pub ip: String,
    pub state: String,
}

pub fn dev_state(ip: Ipv4Addr) -> bool {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("device_states.db");
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT state FROM devices WHERE ip_add = '{}'", ip);
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let q_result = query
                .query_map([], |row| {
                    Ok(devstate{
                        ip: ip.to_string(),
                        state: row.get(0).unwrap(),
                    })
                })
                .unwrap();
            for state in q_result {
                let dev = state.unwrap();
                if dev.state == "up"{
                    return true;
                }else {
                    return false;
                }
            }
        }
        Err(_) => println!("Error opening DB!"),
    }
    return false;
}