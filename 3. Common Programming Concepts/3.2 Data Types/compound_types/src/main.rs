// Compound Types in Rust
// Rust has two primitive compound types:
// 1. Tuple  -> can store multiple values of different types
// 2. Array  -> stores multiple values of same type with fixed length

use std::io;

fn main() {
    // =====================================
    // TUPLES
    // =====================================
    println!("========== Tuples ==========\n");

    // Tuple with explicit type annotation
    // Syntax: (type1, type2, type3)
    let tup: (i32, char, u8) = (-400, 'a', 5);

    println!("Full tuple: {:?}", tup);

    // -------------------------------------
    // Destructuring Tuple
    // -------------------------------------
    // Breaking tuple into separate variables
    let tup2 = (300, 6.2, 'A');

    let (a, b, c) = tup2;

    println!("\nTuple Destructuring:");
    println!("The value of a is {a}");
    println!("The value of b is {b}");
    println!("The value of c is {c}");

    // -------------------------------------
    // Accessing tuple using index
    // -------------------------------------
    let x = (500, 'a', "darshan");

    println!("\nTuple Index Access:");
    println!("First element  -> {}", x.0);
    println!("Second element -> {}", x.1);
    println!("Third element  -> {}", x.2);

    // Note:
    // Tuple index starts from 0
    // x.4 would cause compile error because tuple has only 3 elements

    // =====================================
    // ARRAYS
    // =====================================
    println!("\n========== Arrays ==========\n");

    // Array of months
    // Arrays are fixed-size collections
    let months = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"
    ];

    println!("Months array created with 12 elements.");

    // -------------------------------------
    // Array with explicit type and size
    // Syntax: [datatype; size]
    // -------------------------------------
    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    println!("\nInteger Array:");
    println!("{:?}", nums);

    // -------------------------------------
    // Array with same repeated values
    // Syntax: [value; size]
    // -------------------------------------
    let repeated = [10; 5];

    println!("\nRepeated Value Array:");
    println!("{:?}", repeated);

    println!("First element -> {}", repeated[0]);
    println!("Last element  -> {}", repeated[4]);

    // =====================================
    // USER INPUT ARRAY INDEX EXAMPLE
    // =====================================
    println!("\n========== User Input Example ==========\n");

    println!("Enter month number (1 to 12):");

    let mut month = String::new();

    io::stdin()
        .read_line(&mut month)
        .expect("Failed to read input");

    let index: usize = month
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Safety check to prevent panic
    if index >= 1 && index <= 12 {
        println!("The month is -> {}", months[index - 1]);
    } else {
        println!("Invalid month number! Please enter between 1 and 12.");
    }

    // =====================================
    // Important Notes for Revision
    // =====================================

    // Tuple:
    // - Can store different data types
    // - Fixed size
    // - Use .0 .1 .2 for access

    // Array:
    // - All elements same type
    // - Fixed length
    // - Uses indexing [0], [1], [2]

    // Rust checks array bounds at runtime.
    // Invalid index causes panic.
}
