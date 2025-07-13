#![allow(unreachable_code, unused_labels)]

// It is possible to break for continue inner loop simply calling the keyword
// But loops are nested and want to break or continue outer loop 'label must be used

fn main() {
    'outer: loop {
        println!("Entered into outer loop");
        
        'inner: loop {
            println!("Entered into inner loop");

            // This will break innner (current) loop
            // break;

            // This break outer loop
            break 'outer;

        }
        println!("This is unreachable code")
    }


    // Return value from loop
    let mut counter = 0;
    let result = loop {
        counter+=1;

        if counter == 10 {
            break counter*2; // value should be put after break keyword;
        }
    }; // should be ended with semicolon


    println!("result: {}", result);

}