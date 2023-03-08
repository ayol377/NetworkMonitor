//!
//! 

use std::{fs::{File, create_dir}};
use rusqlite::*;
use tauri::api::path::data_dir;

pub fn initialize_db(){
    let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
    let mut path = path.data_dir;
    create_dir(&path);
    path.push("data.db");
    match File::create(&path) {
        Ok(f) => {
            match Connection::open(path) {
                Ok(conn) => {
                    conn.execute("
                        CREATE TABLE devices (
                        mac TEXT NOT NULL PRIMARY KEY,
                        ip_add TEXT,
                        manufacturer TEXT
                        )
                    ", ()).unwrap();
                }
                Err(_) => print!("Error Adding Table!"),
            }
        },
        Err(e) => println!("Error making DB => {}", e),
    }
}