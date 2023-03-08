//!
//! 

use std::net::Ipv4Addr;


pub struct Device {
    mac: String,
    ip: Ipv4Addr,
    manufacturer: String,
    joindate: String,
}

impl Device {
    pub fn new(mac: String, ip: Ipv4Addr, manufacturer: String, joindate: String) -> Self {
        Self { mac, ip, manufacturer, joindate }
    }

    pub fn mac(&self) -> &str{
        return self.mac.as_str();
    }

    pub fn ip(&self) -> Ipv4Addr{
        return self.ip;
    }

    pub fn manufacturer(&self) -> &str{
        return self.manufacturer.as_str();
    }

    pub fn joindate(&self) -> &str{
        return self.joindate.as_str();
    }
}