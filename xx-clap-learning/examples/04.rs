// flag


use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
}

// cargo run --example 04 -- --verbose 

fn main() {
    let cli = Cli::parse();

    println!("verbose: {:?}", cli.verbose);
}
