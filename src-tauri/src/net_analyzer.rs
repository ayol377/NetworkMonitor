//!
//! 


// Imports
use std::{net::{IpAddr, Ipv4Addr}, os::windows::process::CommandExt};
use serde_json::from_str;
use ipnetwork::{self};
use std::process::Command;
use crate::{structs::Device, database::{is_up, add_device}};

// Constants
const DETACHED_PROCESS: u32 = 0x00000008;
// const DEF_GATEWAY:Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);
// const NET_PREFIX:u8 = 8;


pub fn scan() -> Result<Vec<Device>, String>{
    let mut devices:Vec<Device> = vec![];
    let dev_ip = getip();
    let local_net = getnet();

    // Identify online hosts
    let mut cmd = Command::new("arp");
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
                                    let new_dev:Device = Device{mac: mac.to_string(), ip: ip, manufacturer: "UNKNOWN".to_string(), joindate: "UNKNOWN".to_string()};
                                    // println!("IP: {} => Mac: {}", new_dev.ip(), new_dev.mac());
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

pub fn trace(dest_ip:IpAddr) -> Result<Vec<IpAddr>, String>{
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

pub fn ping(ip: Ipv4Addr){
    let mut cmd = Command::new("ping");
    cmd.creation_flags(DETACHED_PROCESS);
    cmd.arg(format!("{}", ip));
    cmd.arg("-w");
    cmd.arg("-100");
    cmd.arg("-n");
    cmd.arg("1");
    match cmd.status() {
        Ok(_) => println!("OK"),
        Err(_) => println!("Err"),
    }
}

pub fn ping_check(ip: Ipv4Addr) -> bool{
    let mut cmd = Command::new("ping");
    cmd.creation_flags(DETACHED_PROCESS);
    cmd.arg(format!("{}", ip));
    cmd.arg("-w");
    cmd.arg("-100");
    cmd.arg("-n");
    cmd.arg("1");
    let out =  cmd.output().unwrap().stdout;
    let stout = String::from_utf8_lossy(&out);
    if stout.contains("Destination host unreachable") {
        return false;
    }else {
        return true; 
    }
}

pub fn pingscan(){
    let network = getnet();
    for ip in network.iter() {
        tokio::task::spawn(async move {
            let status = ping_check(ip);
            if status {
                is_up(true, ip);
            }else{
                is_up(false, ip);
            }
        });  
    }
    let devs = scan().unwrap();
    for dev in devs{
        add_device(dev);
    }
}