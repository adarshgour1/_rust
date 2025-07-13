// bringing all element into scope
use crate::List::*;

enum List {
    // Cons: Tuple struct that wrap an element and pointer to next element
    Cons(u32, Box<List>),

    // Nil: Signify end of linked list
    Nil,
}

// Methods that can be attached to enum
impl List {

    fn new() -> Self{
        Nil
    }

    // append element in front of list
    fn prepend(self, elem: u32) -> Self {
        Cons(elem, Box::new(self))
    }

    // return len of list
    fn len(&self) -> u32 {
        match self {
            Cons(_, next ) => 1 + next.len(),
            Nil => 0,
        }
    } 

    // return representation of list as heap allocated string
    fn stringigy(&self) -> String {
        match *self {
            Cons(head, ref next ) => format!("{}, {}", head, next.stringigy()),
            Nil => format!("Nil"),
        }
    }

}


fn main() {

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(4);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringigy());
}