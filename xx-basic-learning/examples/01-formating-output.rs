
    // format!: write formatted text to String
    // print!: same as format! but the text is printed to the console (io::stdout).
    // println!: same as print! but a newline is appended.
    // eprint!: same as print! but the text is printed to the standard error (io::stderr).
    // eprintln!: same as eprint! but a newline is appended.



fn main() {

    // Placeholder are bound to index
    println!("{0} + {1} = {2} and {2} - {1} = {0} ", 11, 12, 23);


    // number formatting
    println!("Base 10 (decimal)         :{}", 6124);
    println!("Base 2 (binary)           :{:b}", 6124);
    println!("Base 8 (octal)            :{:o}", 6124);
    println!("Base 16 (hexadecimal)     :{:x}", 6124);



    // left justify
    println!("left jutify number: {:>20}", 24);
    // append with 0
    println!("left jutify number: {:0>20}", 24);
    // append with 1
    println!("left jutify number: {:1>20}", 24);
    
    // right justify
    println!("right jutify number: {:1<20}", 24);
    
    
    // variable literal
    println!("variable left right number: {x:1<width$}", x=  24, width = 23);
    println!("variable left jutify number: {x:1>width$}", x=  24, width = 32);


    // vai surrouding env
    let x = 32;
    let width = 20;

    println!("surrouding right justify number: {x:0>width$}");
    println!("surrouding left justify number: {x:0<width$}");
    

}