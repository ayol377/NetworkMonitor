use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::thread;
use dns_lookup::*;
use tokio::*;
use trust_dns_resolver::*;

pub fn check_gateway_change(){

}

pub fn check_dns_spoof(){
    // let test_hosts = [
    //     "google.com"
    // ];
    // // let resolver = Resolver::new(config::ResolverConfig::cloudflare(), config::ResolverOpts::default()).unwrap();
    // let resolver = Resolver::from_system_conf().unwrap();
    // for domain in test_hosts{
    //     // let def_ips = lookup_host(domain).unwrap();
    //     let response = resolver.lookup_ip(domain).unwrap();
    //     println!("Sec size: {}", response.iter().count());
    //     // println!("UnSec size: {}", def_ips.iter().count());
    // }
}

pub async fn security_coroutine(rate: u64, dns: bool, evil: bool, mitm: bool){
    loop {
        let path = platform_dirs::AppDirs::new(Option::Some("NetSecure"), false).unwrap();
        let mut path = path.data_dir;
        path.push("settings.json");
        let data = fs::read_to_string(&path).await.unwrap();
        let data = data.as_str();
        let settings = json::parse(data).unwrap();
        let email = settings["email"].as_str().unwrap();
        let thread = task::spawn(async move {
            if dns {
                check_dns_spoof();
            }
            if evil {
                check_gateway_change();
            }
        });
        thread::sleep(Duration::from_secs(rate));
    }
}