//!
//! 

use std::net::Ipv4Addr;

pub struct Device {
    pub mac: String,
    pub hostname: String,
    pub ip: Ipv4Addr,
    pub manufacturer: String,
    pub joindate: String,
}

impl Device {
    
    pub fn mac(&self) -> &str{
        return self.mac.as_str();
    }

    pub fn ip(&self) -> Ipv4Addr{
        return self.ip;
    }

    pub fn manufacturer(&self) -> &str{
        return self.manufacturer.as_str();
    }

    pub fn hostname(&self) -> &str{
        return self.hostname.as_str();
    }

    pub fn joindate(&self) -> &str{
        return self.joindate.as_str();
    }
}
