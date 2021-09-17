use cidr::{Ipv4Inet, Ipv6Inet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Serialize, Deserialize)]
struct Log {
    timestamp: f32,
    clientip: String,
    serverip: String,
    method: String,
    url: String,
    status: u32,
    size: u64,
    resp_time: f32,
    http_host: String,
    referer: String,
    user_agent: String,
}

fn print_map_by_size(map: HashMap<String, u64>) {
    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    for item in vec {
        if item.1 < &(1024 * 1024 * 1024) {
            return
        }
        println!("{} {}", item.0, item.1);
    }
}

pub fn process(iterator: std::io::Lines<std::io::BufReader<std::boxed::Box<dyn std::io::Read>>>) {
    let mut total_size = 0;
    let mut ip_map: HashMap<String, u64> = HashMap::new(); // By /24 (IPv4) or /48 (IPv6)
    let mut ua_map: HashMap<String, u64> = HashMap::new();
    let server_ip = "202.141.160.110";

    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).unwrap();
    for line in iterator {
        if term.load(Ordering::Relaxed) {
            println!("Exiting early as SIGTERM or SIGINT is received...");
            break
        }
        match line {
            Err(err) => panic!("cannot read line: {}", err),
            Ok(line) => {
                // Here we replace "\x" to empty string
                // as serde_json does not like this escape sequence and complains a lot.
                let item: Log = serde_json::from_str(&*(line.replace("\\x", ""))).unwrap();
                if item.serverip != server_ip {
                    continue;
                }
                total_size += item.size;
                let client = IpAddr::from_str(&item.clientip).unwrap();
                let client = match client {
                    IpAddr::V4(v4client) => {
                        Ipv4Inet::new(v4client, 24).unwrap().first().to_string()
                    }
                    IpAddr::V6(v6client) => {
                        Ipv6Inet::new(v6client, 48).unwrap().first().to_string()
                    }
                };
                match ip_map.get_mut(&client) {
                    Some(v) => *v += item.size,
                    None => {
                        ip_map.insert(client, item.size);
                    }
                }
                match ua_map.get_mut(&item.user_agent) {
                    Some(v) => *v += item.size,
                    None => {
                        ua_map.insert(item.user_agent, item.size);
                    }
                }
            }
        }
    }
    println!("All requests of IP {}:", server_ip);
    println!("Total Size: {}", total_size);
    println!("IP:");
    print_map_by_size(ip_map);
    println!("UA:");
    print_map_by_size(ua_map);
    // println!("IPs: {}", all_ip_pool.len());
}
