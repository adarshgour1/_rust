#![allow(dead_code)]
#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

fn main() {

    let even = 48;
    let odd = 49;

    let even_struct = EvenNumber::try_from(even);
    let odd_struct = EvenNumber::try_from(odd);

    println!("{:?}", even_struct);
    println!("{:?}", odd_struct);
}
