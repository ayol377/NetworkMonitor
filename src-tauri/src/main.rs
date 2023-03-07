#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Imports
use std::{net::{IpAddr}};
use net_analyzer;

// Modules
mod bootstrapper;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hosts])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn hosts() -> String{

  let mut s_hosts:String = "".to_owned();
  let scan_result = net_analyzer::scan();
  match scan_result {
      Ok(up_hosts) => {
        for ip in up_hosts{
          let r_path = path(ip);
          match r_path {
              Ok(s) => {
                let s = s.as_str();
                let hostline = format!("{} : {}", ip, s);
                let hostline = hostline.as_str();
                print!("{}", hostline);
                s_hosts.push_str(hostline);
              }
              Err(_e) => todo!(),
          }
        }
      },
      Err(_e) => todo!(),
  }
  s_hosts
}

fn path(dest:IpAddr) -> Result<String, String>{
  let path = net_analyzer::trace(dest);
  let mut pathto:String = "".to_owned();
  match path {
      Ok(p) => {
          for ip in p {
              pathto.push_str(" <= ");
              let ipstr = format!("{}", ip);
              let ipstr2 = ipstr.as_str();
              pathto.push_str(ipstr2);
          }
          return Result::Ok(pathto);
      },
      Err(_e) => todo!(),
  }
}


