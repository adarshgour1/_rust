#![allow(overflowing_literals)]
// Rust provide no implicit conversion, but provide explicit conversion

fn main() {
    let decimal = 65.4321f32;

    // Error: no implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer: u8 = decimal as u8;
    let character = integer as char;

    // there is limitation float to char direct conversion  is not possible
    // let character = decimal as char;

    println!("Casting {} -> {} -> {}", decimal, integer, character);

    // 1000  already fits in u16
    println!(" 1000 as u16 is {}", 1000 as u16);

    // 1000 -256 -256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept.
    // while the rest toward the most significant bits (MSB) get truncated.
    println!(" 1000 as u8 is {}", 1000 as u8);

    // -1 + 256 = 255
    println!(" -1 as u8 is {}", -1i8 as u8);

    // for positive number it is same as modulus
    println!("1000 mod 256 is {}", 1000 % 256);

    // Rust keyword as perform saturating cast
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or lower bound, the return value will be equal to bound crossed.

    // 300.0 as 255 as u8
    println!("300.0 as u8 is {}", 300.0 as u8);
    // -100.0 as 0
    println!("-100 as u8 is {}", -100f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
