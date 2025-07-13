fn main() {
    a();
    tuple();
    array();
    reference();
    binding();
}

fn a() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}

// tuple can be destructed in match
fn tuple() {
    let triplet = (3, -2, 4);

    println!("Tell me about: {:?}", triplet);

    match triplet {
        (0, y, z) => println!("first is 0, y: {}, z: {}", y, z),
        (1, ..) => println!("first is 1, rest doesn't matter"),
        (.., 2) => println!("last is 2, rest doesn't matter"),
        (3, .., 4) => println!("firs is 3 and last is 4, rest doesn't matter"),
        _ => println!("doesn't matter what they are",)
    }
}

fn array() {
    let array = [1, -2, 6];

    match array {
        [0, second, third] => println!("first is 0, y: {}, z: {}", second, third),
        [1, _, third] => println!("first is 1, third is {}", third),
        [-1, second, ..]=> println!("first: {}, second: {}, rest ignored", -1, second),
        
        // this will not compile
        // [1, second] => ...
        [3, second, tail @..] => println!("first: 3, second: {second} and rest: {:?}", tail),
        //match in middle
        [first, middle@.., last] => println!("first: {first}, middle: {middle:?} and last: {last}"),
    }
}


fn reference() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}

// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

fn binding() {
    println!("Tell me what type of person you are");

    match age() {
        n @ 0             => println!("I haven't celebrated my first birthday yet, as n is {n}"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n             => println!("I'm an old person of age {:?}", n),
    }
}