// arg by position and by flag

//  cargo run --example 02 -- --help

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

use clap::Parser;

#[derive(Parser)]
// #[command(name= "MyApp")]
// #[command(version = "1.2")]
// #[command(about = "this is MyApp", long_about = None)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    // By default, struct fields define positional arguments:
    two: String,

    #[arg(long)]
    one: String,
}

fn main() {
    let cli = Cli::parse();

    println!("one: {:?}", cli.one);
    println!("two: {:?}", cli.two);
}
