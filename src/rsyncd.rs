use std::collections::HashSet;
use chrono::NaiveDateTime;
use regex::Regex;

static PARSE_STR: &str = "%Y/%m/%d %H:%M:%S";

pub fn process(iterator: std::io::Lines<std::io::BufReader<std::boxed::Box<dyn std::io::Read>>>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<date>\d\d\d\d/\d\d/\d\d \d\d:\d\d:\d\d) \[.+\] send .+ \[(?P<ip>.+)\].+").unwrap();
        static ref START_DATE_UNIX: i64 = NaiveDateTime::parse_from_str("2021/09/01 00:00:00", PARSE_STR).unwrap().timestamp();
        static ref END_DATE_UNIX: i64 = NaiveDateTime::parse_from_str("2021/09/02 00:00:00", PARSE_STR).unwrap().timestamp();
    }

    let mut counter = 0;
    let mut ip_pool: HashSet<String> = HashSet::new();
    for line in iterator {
        match line {
            Err(err) => panic!("cannot read line: {}", err),
            Ok(line) => {
                let caps = RE.captures(line.as_str());
                match caps {
                    None => {
                        // eprintln!("line '{}' does not match.", line)
                    }
                    Some(caps) => {
                        let date = NaiveDateTime::parse_from_str(&caps["date"], PARSE_STR).unwrap();
                        let date_unix = date.timestamp();
                        let ip = &caps["ip"];
                        if *START_DATE_UNIX < date_unix && date_unix < *END_DATE_UNIX {
                            counter += 1;
                            ip_pool.insert(ip.to_string());
                        }
                    }
                }
            }
        }
    }
    println!("Rsync requests:");
    println!("Lines: {}", counter);
    println!("IPs: {}", ip_pool.len());
}