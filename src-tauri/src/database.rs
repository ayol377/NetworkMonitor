//!
//!

use std::net::Ipv4Addr;
use rusqlite::{self, Connection};
use mac_oui::Oui;
use time::{self, OffsetDateTime, format_description};
use crate::{structs::Device, net_analyzer::str_to_ip};

pub fn add_device(dev: Device) {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("data.db");
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT mac FROM devices WHERE mac = '{}'", dev.mac());
            let mut query = conn.prepare(&query.as_str()).unwrap();
            unsafe{
            let mut q_result = query
                .query_map([], |row| {
                    Ok(Device {
                        mac: row.get(0).unwrap(),
                        hostname: "UNKNOWN".to_string(),
                        ip: Ipv4Addr::new(1, 1, 1, 1),
                        manufacturer: "UNKNOWN".to_string(),
                        joindate: "UNKNOWN".to_string(),
                    })
                })
                .unwrap();
            match q_result.next() {
                Some(_) => {
                    let query = format!(
                        "UPDATE devices SET ip_add = '{}' WHERE mac = '{}'",
                        dev.ip(),
                        dev.mac()
                    );
                    conn.execute(query.as_str(), ()).unwrap();
                }
                None => {
                    let query = format!(
                        "INSERT INTO devices (mac, ip_add, manufacturer, hostname) VALUES ('{}', '{}', '{}', '{}')",
                        dev.mac(),
                        dev.ip(),
                        dev.manufacturer(),
                        dev.hostname()
                    );
                    conn.execute(query.as_str(), ()).unwrap();
                    let desc = format!("New device ( {} | {} | {} ) has joined the network", dev.hostname(), dev.mac(), dev.ip());
                    let current_time: OffsetDateTime = OffsetDateTime::now_local().unwrap();
                    let time = format!("{}", current_time.format(&format_description::parse("[hour]-[minute]-[second]").unwrap()).unwrap());
                    let date = format!("{}", current_time.format(&format_description::parse("[year]-[month]-[day]").unwrap()).unwrap());
                    alert( time, date, desc, "info".to_string());
                }
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
                        hostname: row.get(1).unwrap(),
                        ip: str_to_ip(row.get(2).unwrap()),
                        manufacturer: row.get(3).unwrap(),
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

pub fn mf_lookup (mac: String) -> String{
    let oui_db = Oui::default().unwrap();
    let entry_op = oui_db.lookup_by_mac(&mac).unwrap();
    match entry_op {
        Some(entry) => return entry.company_name.to_owned(),
        None => return  "UNKNOWN".to_string(),
    }
}

pub fn alert(time: String, date:String, desc: String, level: String){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("alerts.db");
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("INSERT INTO alerts (time, date, level, desc) VALUES ('{}', '{}', '{}', '{}')",
            time, date, level, desc,);
            conn.execute(&query, ()).unwrap();
        }
        Err(_) => println!("Error opening DB!"),
    }
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

struct Devstate {
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
                    Ok(Devstate{
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

pub struct Alert{
    pub time: String,
    pub date: String,
    pub level: String,
    pub desc: String,
}

pub fn get_alerts() -> Vec<Alert> {
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("alerts.db");
    let mut all_alerts: Vec<Alert> = vec![];
    match Connection::open(path) {
        Ok(conn) => {
            let query = format!("SELECT * FROM alerts");
            let mut query = conn.prepare(&query.as_str()).unwrap();
            let q_result = query
                .query_map([], |row| {
                    Ok(Alert{
                        time: row.get(0).unwrap(),
                        date: row.get(1).unwrap(),
                        level: row.get(2).unwrap(),
                        desc: row.get(3).unwrap(),
                    })
                })
                .unwrap();
            for alert in q_result {
                all_alerts.push(alert.unwrap());
            }
        }
        Err(_) => println!("Error opening DB!"),
    }
    return all_alerts;
}