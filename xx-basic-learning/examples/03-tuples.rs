#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // A tuple can be bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can extracted using indexing
    println!("Long typle first values: {}", long_tuple.0);
    println!("Long typle second values: {}", long_tuple.2);

    // Tuples can be tuples members
    let tuples_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuples_of_tuples);

    // But long tuples (more than 12 element) can not be printed
    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13,14);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("Reversed pair is {:?}", reverse(pair));

    // One element tuples, the comma is required to them apart
    // from literal surrounded by parentheses
    println!("One element tuple: {:?}", (1u32,));
    println!("Just an integer: {:?}", (5i32,));

    // tuple can be destructured into elements
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    println!("{}", matrix);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (first, second) = pair;
    (second, first)
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix:\n({},{})\n({},{})",
            self.0, self.1, self.2, self.3
        )
    }
}
