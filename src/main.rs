use std::net::{IpAddr, Ipv4Addr};
use net_analyzer;

const TEST_IP:IpAddr = IpAddr::V4(Ipv4Addr::new(172,253,118,101));

fn main() {
    let path = net_analyzer::trace(TEST_IP);
    match path {
        Ok(p) => {
            for ip in p {
                println!("|");
                println!("|");
                println!("{}", ip);
            }
        },
        Err(e) => print!("{}", e),
    }
}
