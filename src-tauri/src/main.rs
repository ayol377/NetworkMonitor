//!
//!

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Imports
use std::{fs::{File, create_dir}};
use tauri::PathResolver;
use platform_dirs;

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
    match net_analyzer::scan() {
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
