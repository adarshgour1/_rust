// To convert any type to a String is as simple as implementing the ToString trait for the type.
// Rather than doing so directly, you should implement the fmt::Display trait
// which automatically provides ToString and also allows printing the type as discussed in the section on print!.

use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Circle {
    radius: i32,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "circle of radius is {}", self.radius)
    }
}

impl FromStr for Circle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let circle = Circle { radius: 30 };
    println!("{}", circle.to_string());
    println!("{}", circle);

    // It's useful to convert strings into many types,
    // but one of the more common string operations is to convert them from string to number.
    // The idiomatic approach to this is to use the parse function and either to arrange
    // for type inference or to specify the type to parse using the 'turbofish' syntax.
    // Both alternatives are shown in the following example.
    // This will convert the string into the type specified as long as the FromStr trait is implemented for that type.
    // This is implemented for numerous types within the standard library.

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // To obtain this functionality on a user defined type simply implement the FromStr trait for that type.
    let radius = "    3 ";
    let circle: Circle = radius.parse().unwrap();
    println!("{:?}", circle);
}
