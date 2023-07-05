use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

// Tuples go brrr
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair; // This is making two variables out
    // of the parts of a tuple
    
    (bool_param, int_param) // I guess we don't do "return" to return?
    // Also apparently it doesn't need ;?
}

fn main() {
    let logical: bool = false;
    
    let a_float: f64 = 1.0;
    
    let an_integer: i32 = 5; // Both of these methods work for setting type
    let an_integer_2 = 5i32; // Crazy
    
    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    
    let mut mutable = 12; // A mutable variable's value can be changed
    // mutable = true; // A mutable variable's type cannot be changed
    let mutable = true; // Variables can be overwritten with shadowing
    
    let mut inferred_type = 12; // Defines this variable as i32
    inferred_type = 4294967296i64; // But that was a lie, it actually set it to
    // i64 because it sees in a future line it needs it to be i64
    
    let integer_x: i32 = 0x3AB; // An integer can be expressed using hex,
    // octal, or binary
    // 0x, 0o, and 0b
    
    // Underscores can be inserted in numeric literals to improve readability
    // e.g. 1_000 is the same as 1000 and 0.000_001, is the same as 0.000001
    
    // Rust also supports E-notation, e.g. 1e6, 7.6e-4, and etc.
    
    let test_tuple: (bool, bool, bool) = (true, false, true);
    // Apparently we index tuples with . instead of []
    // Curious to see which way arrays are indexed
    let thingy_bool: bool = test_tuple.0;
    
    // Tuples are also printable with the debug thingy
    println!("{:?}", test_tuple);
    // Long tuples, more than 12 elements, cannot be printed
    
    // To create one element tuples, the comma is requires to tell them
    // apart from a literal surrounded by parentheses
    println!("One element tuple!: {:?}", (5u32,));
    println!("Literally just an integer: {:?}", (5u32));
    
    // Arrays and slices go brrrr
    
    // Arrays' length is known at compile time
    // and is part of their type signature [T; length]
    
    // Slices are similar to arrays 
    // but their length is not known at compile time
    // A slice is a two word object; the first word is a pointer to the data,
    // the second word is the length of the slice
    // The word size is the same as usize, determined by the processor
    // architecture, e.g. 64 bits on an x86-64
    // Slices can be used to borrow a section of an array and have the type
    // signature &[T]
    
    // Fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    
    // All elements can be initialized to the same value
    let ys: [i32; 500] = [69; 500];
    
    // Indexing starts at 0
    println!("{}", xs[3]);
    println!("{}", ys[333]);
    
    // "len" returns the count of elements in an array
    println!("Number of elements in this array: {}", xs.len());
    
    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    
    // Arrays can be automatically borrowed as slices
    analyze_slice(&xs);
    
    // Slices can point to a section of an array
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    analyze_slice(&ys[0..4]);
    
    // Example of an empty array and an empty slice
    let empty_array: [u32; 0] = [];
    let empty_slice: &[u32] = &[];
    
    // Arrays can be safely accessed using ".get",
    // which returns an "Option". this can be matched as shown below, or used
    // with ".expect()" if you would like the program to exit with a nice
    // message instead of happily continue
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        // It took me a minute to get what "match" does but I think I get it now
        // It seems to essentially be a switch statement
        // If something, it does that first thing
        // and if none then it does that second
        // The first one gives us the value as well so we can use it
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}