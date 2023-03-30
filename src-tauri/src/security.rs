use std::time::Duration;
use std::thread;
use futures::future;
use tokio::*;

pub fn check_gateway_change(){

}

pub fn check_dns_spoof(){
    
}

pub async fn security_coroutine(rate: u64, dns: bool, evil: bool, mitm: bool){
    loop {
        task::spawn(async move {
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