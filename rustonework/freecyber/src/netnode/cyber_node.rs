use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Serialize, Deserialize)]
pub struct Node {
    ip: String,
    port: u16,
    country: String,
    city: String,
    speed: usize, // ms
    p_type: String,
    anonymity: String, // 匿名度
    last_check: usize, // ms
}

impl Node {
    pub fn new() -> Self {
        Node {
            ip: "".to_string(),
            port: 0,
            country: "".to_string(),
            city: "".to_string(),
            speed: 0,
            p_type: "".to_string(),
            anonymity: "".to_string(),
            last_check: 0,
        }
    }
    pub fn set_ip(&mut self, ip: &str, port: u16) -> &mut Self {
        self.ip = ip.to_string();
        self.port = port;
        self
    }
    pub fn ip(&self) -> &str {
        &self.ip
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn set_city_country(&mut self, city: &str, country: &str) -> &mut Self {
        self.city = city.to_string();
        self.country = country.to_string();
        self
    }
    pub fn set_speed(&mut self, speed: usize) -> &mut Self {
        self.speed = speed;
        self
    }
    pub fn set_type(&mut self, type_p: &str) -> &mut Self {
        self.p_type = type_p.to_string();
        self
    }
    pub fn set_anonymity(&mut self, anonymity: &str) -> &mut Self {
        self.anonymity = anonymity.to_string();
        self
    }
    pub fn set_last_check(&mut self, last_check: usize) -> &mut Self {
        self.last_check = last_check;
        self
    }
}

// 为了能够排序，需要实现 `PartialOrd` 和 `Ord` trait
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 根据 `speed` 字段升序排序
        self.speed.cmp(&other.speed)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed
    }
}
