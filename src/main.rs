// cargo run --release -- nginx nginx_log_file
// cargo run --release -- rsyncd rsync_log_file
// xzcat xxx.xz | cargo run --release -- rsyncd
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{self, BufRead};
mod processor {
    pub mod nginx_json;
    pub mod nginx;
    pub mod rsyncd;
}
mod r#struct;
use r#struct::Cli;
use structopt::StructOpt;

enum FileType {
    Nginx,
    Rsyncd,
    NginxJson,
}

fn main() {
    let args = Cli::from_args();
    let file_type: FileType = match args.r#type.as_str() {
        "nginx" => FileType::Nginx,
        "rsyncd" => FileType::Rsyncd,
        "nginx_json" => FileType::NginxJson,
        _ => panic!("Wrong file type!"),
    };
    let file: Box<dyn std::io::Read + 'static> = match args.filename {
        Some(ref filename) => match File::open(&filename) {
            Err(err) => panic!("cannot open {}: {}", filename.display(), err),
            Ok(file) => Box::new(file),
        },
        None => Box::new(io::stdin()),
    };
    let reader = io::BufReader::new(file);
    let iterator = reader.lines();
    match file_type {
        FileType::Nginx => processor::nginx::process(iterator),
        FileType::Rsyncd => processor::rsyncd::process(iterator),
        FileType::NginxJson => processor::nginx_json::process(iterator, args),
    }
}
