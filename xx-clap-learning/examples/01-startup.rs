// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "This is my first clap app")]
struct Cli {
    /// First Name of person
    #[arg(short, long)]
    firstname: String,

    /// Last Name of person
    #[arg(short, long)]
    lastname: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli);
}

impl std::fmt::Display for Cli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "first-name: {}", self.firstname)?;
        if let Some(lastname) = &self.lastname {
            write!(f, " and last-name: {}", lastname)?;
        }
        Ok(())
    }
}
