use std::{env::home_dir, fs};

use clap::{Parser, command};

mod github;

#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
struct Cli {
    /// Github token
    #[arg(long)]
    token: Option<String>,
}

fn main() {
    let mut cli = Cli::parse();

    if let None = cli.token {
        cli.token = read_gh_token()
    }

    println!("{:?}", cli);
}

fn read_gh_token() -> Option<String> {
    fs::read_to_string(home_dir()?.join(".netrc"))
        .ok()?
        .lines()
        .find_map(|line| {
            if line.starts_with("  password ") {
                Some(line.trim_start_matches("  password ").to_owned())
            } else {
                None
            }
        })
}
