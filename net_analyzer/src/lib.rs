use std::net::IpAddr;
use tracert::{self, ping::{Pinger, PingStatus}};


pub fn scan(gateway:IpAddr){
    todo!();
}

pub fn ping(dest_ip:IpAddr) -> Result<PingStatus, String>{
    let ip_pinger = Pinger::new(dest_ip);
            match ip_pinger {
                Ok(p) => {
                    let ip_ping = p.ping();
                    match ip_ping {
                        Ok(p) => return Result::Ok(p.status),
                        Err(e) => return Result::Err(e),
                    }
                }
                Err(e) => return Result::Err(e),
            }
}

pub fn trace(dest_ip:IpAddr) -> Result<Vec<IpAddr>, String>{
    let mut path_to_dest = vec![];
    let ip_tracer:Result<tracert::trace::Tracer, String> = tracert::trace::Tracer::new(dest_ip);
    match ip_tracer {
        Ok(t) => {
            println!("Tracer Setup Successfull");
            let ping_r = ping(dest_ip);
            match ping_r {
                Ok(ps) =>{
                    match ps {
                        PingStatus::Done => println!("Host Up!"),
                        PingStatus::Error => return Result::Err("Ping Error!".to_string()),
                        PingStatus::Timeout => return Result::Err("Destination Host Cant be Reached".to_string()),
                    }
                }
                Err(e) => return Result::Err(e),
            }
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