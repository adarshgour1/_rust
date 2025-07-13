// list of input

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Vec<String>,
}

// cargo run --example 03 -- --name adarsh -n sham -n=gour 
fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
}
