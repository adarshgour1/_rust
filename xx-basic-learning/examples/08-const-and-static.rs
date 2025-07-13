// Globals are declared outside of scopes.
static LANGAUGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in main thread
    println!("This is {}", LANGAUGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error cant modify const
    // THRESHOLD = 5;
    // LANGAUGE = "c++";
}
