#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

const TEST_IP:IpAddr = IpAddr::V4(Ipv4Addr::new(172,253,118,101));

use std::net::{IpAddr, Ipv4Addr};
use net_analyzer;

fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![tracedump])
    //.invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
async fn tracedump() -> String{
  let path = net_analyzer::trace(TEST_IP);
  let mut pathto:String = "".to_owned();
  match path {
      Ok(p) => {
          for ip in p {
              pathto.push_str("| <br>");
              pathto.push_str("| <br>");
              let ipstr = format!("{} <br>", ip);
              let ipstr2 = ipstr.as_str();
              pathto.push_str(ipstr2);
          }
          print!("{}", pathto);
          print!("Done!");
      },
      Err(_e) => return format!("Failed"),
  }
  pathto
}
