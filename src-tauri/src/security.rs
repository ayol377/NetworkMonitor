use std::time::Duration;
use std::thread;
use tokio::*;

pub fn check_gateway_change(){

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