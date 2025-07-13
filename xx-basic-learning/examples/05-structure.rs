#![allow(dead_code)]


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Unit struct
struct Unit;

// A tuple struct with name
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// struct can be used as field of another struct
struct Rectangle {
    // rectangle can be specified using top left and bottom right poin
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create a struct with field init shorthand
    let name = String::from("Adarsh");
    let age = 25;
    let person = Person { name, age };

    // print debug struct
    println!("{:?}", person);

    // Instantiate point
    let point = Point { x: 2.5, y: 2.3 };
    let another_point = Point { x: 10.2, y: 1f32 };

    // Access field of point
    println!("cordinate of point: {}, {}", point.x, point.y);

    // Make new point using update struct syntax
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // y of bottom_right will be same as another_point
    println!("cordinate of point: {}, {}", bottom_right.x, bottom_right.y);

    // Destructure using let binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    // struct initialization is expression too
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Instanciate unit struct
    let _unit = Unit;

    // Instatiate tuple struct
    let _tuple = Pair(1, 2.3);

    // Access fields of tuple struct
    println!("Pair integer: {:?}, Pair decimal: {:?}", _tuple.0, _tuple.1);

    // destructure tuple struct
    let Pair(number, decimal) = _tuple;

    println!("number: {:?}, decimal: {:?}", number, decimal);
}
