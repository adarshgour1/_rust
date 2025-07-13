#![allow(dead_code)]
#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self { value: value }
    }
}


#[derive(Debug)]
struct Circle {
    radius: i32
}

impl Into<Circle> for i32  {
    fn into(self) -> Circle {
        Circle { radius: self }
    }
}

fn main() {

    let integer = 45;
    let number = Number::from(integer);

    println!("{:?}", number);
    
    let number2: Number = integer.into();
    println!("{:?}", number2);
    
    
    
    let r = 32;
    let circle: Circle = r.into();
    println!("{:?}", circle);

    // Below line will be error
    // let circle2 = Circle::from(r);



}

// From and Into are designed to be complementary. 
// We do not need to provide an implementation for both traits. 
// If you have implemented the From trait for your type, 
// Into will call it when necessary. Note, however, that the converse is not true: 
// implementing Into for your type will not automatically provide it with an implementation of From.

