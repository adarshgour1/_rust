fn main() {
    a();
    b();
    c();
}


// iter() iterator borrow element, and keep collection untouched
fn a() {
    let names = vec![
        "Bob",
        "Frank",
        "Ferris",
    ];

    for name in names.iter() {
        match name {
            // Try to uncomment line
            // "Ferris" => println!("There is a rusticean among us"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

// into_iter() consume the data, collection will not be valid after it. It has been moved within a loop
fn b() {
    let names = vec![
        "Bob",
        "Frank",
        "Ferris",
    ];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rusticean among us"),
            _ => println!("Hello {}", name),
        }
    }

    // Try to uncomment will give an error
    // println!("names: {:?}", names);
}

// iter_mut() borrow collection mutably, allowing it to modified in a place
fn c() {
    let mut names = vec![
        "Bob",
        "Frank",
        "Ferris",
    ];

    for name in names.iter_mut() {
        // here updating name in place
        *name = match name {

            // try to remove mut keyword
            &mut "Ferris" => "There is a rusticean among us",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}