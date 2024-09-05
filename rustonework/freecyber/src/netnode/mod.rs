pub mod cyber_node;

use crate::Node;
use clap::Parser;
use reqwest;
use scraper::{Html, Selector};
use std::str::FromStr;

#[derive(Parser, Debug)]
struct FreeProxyWorld {
    #[arg(long = "http")]
    pub http: bool,

    #[arg(long = "socks5")]
    pub socks5: bool,

    #[arg(short = 'c', long = "country", default_value = "")]
    pub country: String,
}

pub async fn get_nodes_from_free_proxy_world() -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let proxy_config = FreeProxyWorld::parse();

    let url_site = "https://www.freeproxy.world";

    let country = proxy_config.country;
    let mut url = "".to_string();

    if proxy_config.http {
        url = format!(
            "{}/?type=http&anonymity=4&country={}&speed=&port=&page=1",
            url_site, country
        );
    }
    if proxy_config.socks5 {
        url = format!(
            "{}/?type=socks5&anonymity=4&country={}&speed=&port=&page=1",
            url_site, country
        );
    }
    if !proxy_config.http && !proxy_config.socks5 {
        url = format!(
            "{}/?type=&anonymity=4&country=&speed=&port=&page=1",
            url_site
        );
    }

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await?;
    let text = res.text().await?;

    let nodes = get_nodes_from_html(&text);

    let nodes = get_nodes_from_vec(nodes).unwrap();

    Ok(nodes)
}
fn get_nodes_from_vec(nodes: Vec<Vec<String>>) -> Option<Vec<Node>> {
    let mut nodes_st = Vec::new();

    for node in &nodes {
        let ip = node.get(0).unwrap();

        if ip.len() == 0 {
            continue;
        }
        let mut m_node = Node::new();

        if let Some(port) = node.get(1) {
            let port = u16::from_str(port).expect("TODO: panic message");
            m_node.set_ip(ip, port);
        }

        let mut ct = "".to_string();
        if let Some(city) = node.get(3) {
            ct = city.to_string();
        }
        let mut cc = "".to_string();
        if let Some(country) = node.get(2) {
            cc = country.to_string();
        }
        m_node.set_city_country(&ct, &cc);

        if let Some(speed) = node.get(4) {
            let len = speed.len();
            let speed = speed.get(0..(len - 2)).map(|a| a.to_string()).unwrap();

            let speed = usize::from_str(&speed).unwrap();
            m_node.set_speed(speed);
        }

        if let Some(types) = node.get(5) {
            m_node.set_type(types);
        }
        if let Some(an) = node.get(6) {
            m_node.set_anonymity(an);
        }
        if let Some(las) = node.get(7) {
            let mut ms = 0usize;

            for n in las.split('.').collect::<Vec<&str>>() {
                let len = n.len();
                if n.contains('h') {
                    let h = n.get(0..(len - 1));
                    ms += 1000 * 60 * 60 * usize::from_str(h.unwrap()).unwrap();
                }
                if n.contains("minutes") {
                    let m = n.get(0..len - 7);
                    ms += 1000 * 60 * usize::from_str(m.unwrap()).unwrap();
                }
                if n.contains("seconds") {
                    let sec = n.get(0..len - 7);
                    ms += 1000 * usize::from_str(sec.unwrap()).unwrap();
                }
            }
            m_node.set_last_check(ms);
        }

        if m_node.ip().len() > 0 {
            nodes_st.push(m_node);
        }
    }

    Some(nodes_st)
}

fn get_nodes_from_html(html: &str) -> Vec<Vec<String>> {
    let doc = Html::parse_document(html);

    let tr_selector = Selector::parse("tr").unwrap();

    let td_selector = Selector::parse("td").unwrap();

    let mut nodes = Vec::new();

    for node in doc.select(&tr_selector) {
        let mut n = Vec::new();

        for td_i in node.select(&td_selector) {
            let text = td_i.text().collect::<Vec<_>>();
            let mut s = String::new();
            for e in text {
                s.push_str(e);
            }
            let s = s.replace(" ", "").replace("\n", "");
            n.push(s);
        }
        if n.len() > 0 {
            nodes.push(n);
        }
    }
    nodes
}
