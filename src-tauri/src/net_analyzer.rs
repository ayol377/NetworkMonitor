//!
//! 

// Imports
use std::{net::{IpAddr, Ipv4Addr}};
use serde_json::from_str;
use tracert::{self, ping::{Pinger}};
use ipnetwork::{self, Ipv4Network};
use std::process::Command;
use crate::structs::Device;

// Constants
const DEF_GATEWAY:Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);
const NET_PREFIX:u8 = 8;


pub fn scan(dev_ip: Ipv4Addr) -> Result<Vec<Device>, String>{
    let mut devices:Vec<Device> = vec![];
    let mut possible_hosts:Vec<Ipv4Addr> = vec![];


    let local_net = Ipv4Network::new(DEF_GATEWAY, NET_PREFIX).unwrap();
    let netsize = local_net.size() - 1;
    for n in 0..netsize {
        possible_hosts.push(local_net.nth(n).unwrap());
    }

    // Identify online hosts
    let mut cmd = Command::new("arp");
    cmd.arg("-a");
    cmd.arg("-N");
    cmd.arg(format!("{}", dev_ip));
    //cmd.arg("-a");

    match cmd.output() {
        Ok(o) =>{
            match String::from_utf8(o.stdout){
                Ok(d) => {
                    let mut data = d.split_ascii_whitespace();
                    for n in 0..9{
                        data.next();
                    }
                    while true {
                        match data.next(){
                            Some(x1) => {
                               let mac = data.next().unwrap();
                                data.next();
                                let mut ip_iter = x1.split(".");
                                let mut octs:Vec<u8> = vec![];
                                for _n in 0..4{
                                    let oct:u8 = from_str(ip_iter.next().unwrap()).unwrap();
                                    octs.push(oct); 
                                }
                                let ip = Ipv4Addr::new(octs[0],octs[1],octs[2],octs[3]);
                                if local_net.contains(ip){
                                    let new_dev:Device = Device::new(mac.to_string(), ip, "UNKNOWN".to_string(), "".to_string());
                                    println!("IP: {} => Mac: {}", new_dev.ip(), new_dev.mac());
                                    devices.push(new_dev);
                                    
                                }
                                
                            },
                            None => break,
                        }
                        
                    }
                },
                Err(_) => todo!(),
            }
        },
        Err(_) => println!("Error!"),
    }
    // for dev in &devices{
    //     println!("{} => {}", dev.ip(), dev.mac());
    // }
    return Result::Ok(devices);
}

pub fn trace(dest_ip:IpAddr) -> Result<Vec<IpAddr>, String>{
    let mut path_to_dest = vec![];
    let ip_tracer:Result<tracert::trace::Tracer, String> = tracert::trace::Tracer::new(dest_ip);
    match ip_tracer {
        Ok(t) => {
            println!("Tracer Setup Successfull");
            let ip_trace:Result<tracert::trace::TraceResult, String> = t.trace();
            match ip_trace{
                
                Ok(r) => {
                    for ip_node in r.nodes {
                        path_to_dest.push(ip_node.ip_addr);
                    }
                },
                Err(e) => return Result::Err(e),
            }
        },
        Err(e) => return Result::Err(e),
    }
    return Result::Ok(path_to_dest);
}