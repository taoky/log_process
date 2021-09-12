// cargo run --release -- nginx nginx_log_file
// cargo run --release -- rsyncd rsync_log_file
// xzcat xxx.xz | cargo run --release -- rsyncd
#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
mod nginx;
mod rsyncd;
mod nginx_json;

enum FileType {
    Nginx,
    Rsyncd,
    NginxJson
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: Option<&String>;
    let file_type: FileType;
    match args.len() {
        2 | 3 => {
            match args[1].as_str() {
                "nginx" => file_type = FileType::Nginx,
                "rsyncd" => file_type = FileType::Rsyncd,
                "nginx_json" => file_type = FileType::NginxJson,
                _ => panic!("Wrong file type!")
            }
            if args.len() == 3 {
                filename = Some(&args[2]);
            } else {
                filename = None;
            }
        }
        _ => {
            println!("Usage: {} [nginx|rsyncd|nginx_json] [filename]", args[0]);
            return
        }
    }
    let file: Box<dyn std::io::Read + 'static> = match filename {
        Some(filename) => match File::open(filename) {
            Err(err) => panic!("cannot open {}: {}", filename, err),
            Ok(file) => Box::new(file)
        },
        None => Box::new(io::stdin())
    };
    let reader = io::BufReader::new(file);
    let iterator = reader.lines();
    match file_type {
        FileType::Nginx => nginx::process(iterator),
        FileType::Rsyncd => rsyncd::process(iterator),
        FileType::NginxJson => nginx_json::process(iterator),
    }
}
