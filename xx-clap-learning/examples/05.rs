// optional arg
use clap::Parser;

// cargo run --example 05 -- Adarsh
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
