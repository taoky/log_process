use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub r#type: String,
    #[structopt(parse(from_os_str))]
    pub filename: Option<PathBuf>,
    #[structopt(long, short)]
    pub server: Option<String>,
}
