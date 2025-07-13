fn main() {
    // Fixed-sized array (type signature is superfluous)
    let xs: [i32; 5] = [1,2,3,4,5];

    // All element can initialize to same value
    let ys: [i32; 500] = [0; 500];

    // Indexing start at 0
    println!("First element of array: {}", xs[0]);
    println!("Second element of array: {}", xs[1]);

    // len return the count of element in the array
    println!("Number of element in array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&xs));

    // Arrays can be automatically borrows as slice
    analyze_slice(&ys);


    // Slice can point to section of array
    // They are of the form [starting_index..ending_index]
    // `starting_index` is first position of slice
    // `ending_index` is one more than the last position
    println!("Borrow section of array as slice");
    analyze_slice(&ys[0..50]);

    // Example of empty slice `&[]`
    let empty_slice: [u32; 0] = [];
    assert_eq!(&empty_slice, &[]);
    assert_eq!(&empty_slice, &[][..]);

    // Array can be safely access using get method which return option
    for i in 0..xs.len() + 1 { // +1 for None case
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i , xval),
            None => println!("Slow down you have already consumed the array"),
        }
    }

    // Out of bound indexing on array with constant value cause compile time error
    // println!("{}", xs[5]);

    // Out of bound indexing on slice causes runtime error
    // println!("{}", xs[..][5]);


}

fn analyze_slice(slice: &[i32]) {
    println!("First element of slice is {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}