//!
//! 

use std::{fs::{File, create_dir, self}};
use rusqlite::*;
use serde_json::*;

pub fn strap(){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let path = path.data_dir;
    match create_dir(&path){
        Ok(_) => println!("main dir made"),
        Err(_) => println!("dir failed"),
    }

    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure/data"), false).unwrap();
    let mut path = path.data_dir;
    match create_dir(&path){
        Ok(_) => println!("data dir made"),
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
                "email": "",
                "dns": true,
                "mitm": true,
                "eviltwin": true,
                "cloudbackup": false
            });
            fs::write(path, data.to_string()).unwrap();
        },
        Err(e) => println!("Error making Settings File => {}", e),
    }

}