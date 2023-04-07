//!
//! 


// Imports
use std::{net::{IpAddr, Ipv4Addr}, os::windows::process::CommandExt, thread, time::Duration};
use futures::future;
use serde_json::from_str;
use ipnetwork::{self};
use tokio::process::Command;
use crate::{structs::Device, database::{add_device}};

use crate::UP_DEVS;

// Constants
const DETACHED_PROCESS: u32 = 0x00000008;
// const DEF_GATEWAY:Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);
// const NET_PREFIX:u8 = 8;


pub fn scan() -> Result<Vec<Device>, String>{
    let mut devices:Vec<Device> = vec![];
    let dev_ip = getip();
    let local_net = getnet();

    // Identify online hosts
    let mut cmd = std::process::Command::new("arp");
    cmd.creation_flags(DETACHED_PROCESS);
    cmd.arg("-a");
    cmd.arg("-N");
    cmd.arg(format!("{}", dev_ip));
    //cmd.arg("-a");

    match cmd.output() {
        Ok(o) =>{
            match String::from_utf8(o.stdout){
                Ok(d) => {
                    let mut data = d.split_ascii_whitespace();
                    for _ in 0..9{
                        data.next();
                    }
                    loop {
                        match data.next(){
                            Some(x1) => {
                                let mac = data.next().unwrap();
                                data.next();
                                let ip = str_to_ip(x1.to_string());
                                if local_net.contains(ip){
                                    let new_dev:Device = Device{mac: mac.to_string(), ip, manufacturer: "DUMMY".to_string(), joindate: "DUMMY".to_string(), hostname: "DUMMY".to_string()};
                                    if new_dev.mac() != "ff-ff-ff-ff-ff-ff"{
                                        devices.push(new_dev);
                                    }
                                    
                                    
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

    return Result::Ok(devices);
}

pub fn get_next_hop(dest_ip:IpAddr) -> Result<Vec<IpAddr>, String>{
    todo!();
}

pub fn getip() -> IpAddr {
    let mut dev_ip:IpAddr = IpAddr::V4(Ipv4Addr::new(1,1,1,1));
    for iface in if_addrs::get_if_addrs().unwrap(){
      if iface.is_link_local(){break;}
      if iface.is_loopback(){break;}
      if iface.addr.ip().is_ipv4() {
        dev_ip = iface.addr.ip();
      }
    }
    return dev_ip;
}

pub fn getmask() -> IpAddr {
    let mut mask = Ipv4Addr::new(0, 0, 0, 0);
    let ip = getip();
    for iface in if_addrs::get_if_addrs().unwrap(){
        if iface.addr.ip() == ip {
          match iface.addr {
            if_addrs::IfAddr::V4(ipv4) => {
                mask = ipv4.netmask;
            },
            if_addrs::IfAddr::V6(_) => println!("Something went wrong!"),
        }
        }
      }
    return IpAddr::V4(mask);
}

pub fn getnet() -> ipnetwork::Ipv4Network {
    let ip = getip();
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    let ip = ipnetwork::IpNetwork::network(&net);
    let net = ipnetwork::IpNetwork::with_netmask(ip, getmask()).unwrap();
    match net {
        ipnetwork::IpNetwork::V4(network) => return network,
        ipnetwork::IpNetwork::V6(_) => {
            println!("Something whent wrong");
            return ipnetwork::Ipv4Network::new(Ipv4Addr::new(0,0,0,0), 0).unwrap();
        },
    }
}

pub fn str_to_ip(ip: String) -> Ipv4Addr {
    let mut ip = ip.split(".");
    let mut octs:Vec<u8> = vec![];
    for _n in 0..4{
        let oct:u8 = from_str(ip.next().unwrap()).unwrap();
        octs.push(oct); 
    }
    let ip = Ipv4Addr::new(octs[0],octs[1],octs[2],octs[3]);
    return ip;
}

pub async fn ping_check(ip: Ipv4Addr) -> bool{
    let mut cmd = Command::new("ping");
    cmd.creation_flags(DETACHED_PROCESS);
    cmd.arg(format!("{}", ip));
    cmd.arg("-w");
    cmd.arg("100");
    cmd.arg("-n");
    cmd.arg("1");
    match cmd.output().await{
        Ok(o) => {
            let stout = String::from_utf8_lossy(&o.stdout);
            if stout.contains("Destination host unreachable") {
                return false;
            }else if  stout.contains("Request timed out") {
                return false; 
            }else{
                return true;
            }
        },
        Err(_) => return false,
    }
    
}

pub async fn pingscan(rate: u64){
    loop {
        unsafe{UP_DEVS.clear();}
        let network = getnet();
        let mut pingtasks = Vec::new();
        for ip in network.iter() {
            let pingtask = tokio::task::spawn(async move {
                // println!("pinging {}", ip);
                let status = ping_check(ip).await;
                if status {
                    println!("{} is up!", ip);
                    unsafe {UP_DEVS.push(ip);}
                    
                }else{
                    // println!("{} is down!", ip);
                }
                });
            pingtasks.push(pingtask);
            if pingtasks.iter().count() >= 1000 {
                future::join_all(pingtasks).await;
                pingtasks = vec![];
            }
        }
        let devs = scan().unwrap();
        for dev in devs{
            add_device(dev);
        }
        println!("~~~~~UP DEVICES~~~~~");
        unsafe{
            for dev in &UP_DEVS{
                println!("{}", dev);
            }
        }
        println!("~~~~~~~~~~~~~~~~~~~~");
        future::join_all(pingtasks).await;
        println!("all pinged, now waiting!");
        thread::sleep(Duration::from_secs(rate));
    }
}