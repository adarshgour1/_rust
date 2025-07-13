// Fn: the clouser uses capture value by reference (&T)
// FnMut: the clouser uses capture value by mutable reference (&mut T)
// FnOnce: the closure uses the capture value by value (T)

fn apply<F> (f: F) where  F: FnOnce() {
    f();
}

fn applly_to_3<F> (f: F) ->i32 where F: Fn(i32) ->i32 {
    f(3)
}


fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();


    // Capture 2 variable: greeting by ref and farewell by value.
    let dairy = || {
        println!("I said {}", greeting);

        // mutation forces `farewell` to be captured by mutable ref.
        farewell.push_str("!!!");
        println!("then I screamed {}.",farewell);
        println!("now I can sleep");

        mem::drop(farewell);

    };
    apply(dairy);
    // apply(dairy); // error

    let double = |x| x*2;
    println!("3 doubled: {}", applly_to_3(double));
    println!("3 doubled: {}", applly_to_3(double));

    apply(function);
    // applly_to_3(function); // error 
}


fn function() {
    println!("This function can be passed as clouser if it fits in trait bound condition");
}