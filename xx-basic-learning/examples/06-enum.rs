#![allow(dead_code)]
enum WebEvent {
    // unit like enum
    PageLoad,
    PageUnload,

    // tuple like enum
    KeyPress(char),
    Paste(String),

    // c-struct like enum
    Click { x: i32, y: i32 },
}

// Alias enum
type Event = WebEvent;

// A function take enum as an argument and return nothing
fn inspect(web_event: Event) {
    // bringing into scope
    use WebEvent::{PageLoad, PageUnload};

    match web_event {
        PageLoad => println!("page loaded"),
        PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("key is pressed: '{}", c),
        WebEvent::Paste(s) => println!("pasted: \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at (x,y): ({},{})", x, y),
    }
}
fn main() {
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let press = WebEvent::KeyPress('c');
    let paste = WebEvent::Paste("I am learing rust".to_string());
    let click = WebEvent::Click { x: 20, y: 30 };


    inspect(load);
    inspect(unload);
    inspect(press);
    inspect(paste);
    inspect(click);
}


// c like enum
// enum with implecit descriminator (start at 0)
enum Number {
    Zero,
    One,
    Two,
    Three,
}

// enum with explicit descriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}