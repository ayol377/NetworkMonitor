//!
//! 

// Imports
use std::{net::{IpAddr, Ipv4Addr}, time::Duration};
use tracert::{self, ping::{Pinger}};
use ipnetwork::{self, Ipv4Network};

// Constants
const DEF_GATEWAY:Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);
const net_prefix:u8 = 8;

pub fn scan() -> Result<Vec<IpAddr>, String>{
    let mut up_hosts = vec![];
    let mut possible_hosts:Vec<Ipv4Addr> = vec![];

    // Find all IP addresses of network
    match Ipv4Network::new(DEF_GATEWAY, net_prefix){
        Ok(local_net) => {
            let net_size = local_net.size() - 1;
            for x in 0..net_size{
                possible_hosts.push(local_net.nth(x).unwrap())
            }
        },
        Err(e) => println!("{}", e),
    }

    // Identify online hosts
    for ip in  possible_hosts{

    }


    return Result::Ok(up_hosts);
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