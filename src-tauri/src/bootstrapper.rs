//!
//! 

use std::{fs::{File, create_dir}};
use rusqlite::*;

pub fn initialize_db(){
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