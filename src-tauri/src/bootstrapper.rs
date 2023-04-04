//!
//! 

use std::{fs::{File, create_dir, self}};
use rusqlite::*;
use serde_json::*;
use crate::net_analyzer::getnet;

pub fn strap(){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    match create_dir(&path){
        Ok(_) => println!("dir made"),
        Err(_) => println!("dir failed"),
    }
    
    path.push("data.db");
    match File::create(&path) {
        Ok(_) => {
            match Connection::open(path) {
                Ok(conn) => {
                    conn.execute("
                        CREATE TABLE devices (
                        mac TEXT NOT NULL PRIMARY KEY,
                        hostname TEXT,
                        ip_add TEXT,
                        manufacturer TEXT,
                        joindate TEXT
                        )
                    ", ()).unwrap();
                }
                Err(_) => print!("Error Adding Table!"),
            }
        },
        Err(e) => println!("Error making DB => {}", e),
    }

    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    path.push("alerts.db");
    match File::create(&path) {
        Ok(_) => {
            match Connection::open(path) {
                Ok(conn) => {
                    conn.execute("
                    CREATE TABLE alerts (
                        time TEXT,
                        date TEXT,
                        level TEXT,
                        desc TEXT
                        )
                    ", ()).unwrap();
                }
                Err(_) => print!("Error Adding Table!"),
            }
        },
        Err(e) => println!("Error making DB => {}", e),
    }

    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    path.push("settings.json");
    match File::create(&path) {
        Ok(_) => {
            let data = json!({
                "dns": true,
                "mitm": false,
                "eviltwin": false,
                "cloudbackup": true
            });
            fs::write(path, data.to_string()).unwrap();
        },
        Err(e) => println!("Error making Settings File => {}", e),
    }

}