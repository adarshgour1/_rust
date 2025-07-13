// For example, if you have arguments of specific values you want to test for, you can derive ValueEnum (any PossibleValue builder function can be used as a #[value] attribute on enum variants).

// This allows you specify the valid values for that argument. If the user does not use one of those specific values, they will receive a graceful exit with error message informing them of the mistake, and what the possible valid values are

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run swiftly
    Fast,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    Slow,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Fast => {
            println!("Hare");
        }
        Mode::Slow => {
            println!("Tortoise");
        }
    }
}
