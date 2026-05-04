// Control Flow in Rust
// if, else if, else
//
// Used to run code based on conditions.
//
// Important:
// Rust requires condition to be bool (true/false)

fn main() {
    // =====================================
    // Basic if else Example
    // =====================================

    let mut number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // -------------------------------------
    // WRONG Example
    // -------------------------------------

    // if number {
    //     println!("{number}");
    // }

    // ERROR:
    // expected bool, found integer

    // Rust does NOT treat:
    // 0 = false
    // non-zero = true
    // like C/C++/JavaScript

    // Correct way:
    if number != 0 {
        println!("Number is not zero");
    }

    // =====================================
    // else if ladder
    // =====================================

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number not divisible by 4, 3, or 2");
    }

    // Only first true condition runs

    // =====================================
    // if as Expression in let
    // =====================================

    let condition = true;

    let value = if condition { 5 } else { 6 };

    println!("Value = {value}");

    // =====================================
    // Both branches must return same type
    // =====================================

    // let value2 = if condition { 5 } else { "six" };

    // ERROR:
    // expected integer, found &str

    // Correct examples:

    let value3 = if condition { 100 } else { 200 };
    println!("Value3 = {value3}");

    let word = if condition { "yes" } else { "no" };
    println!("Word = {word}");

    // =====================================
    // Updating mutable variable
    // =====================================

    number = 12;

    if number > 10 {
        println!("Updated number is greater than 10");
    }
}



// =====================================
// Notes for Revision
// =====================================

// 1. Basic Syntax:
//
// if condition {
//     code
// } else {
//     code
// }

// 2. Condition must be bool

// Correct:
// if x > 5

// Wrong:
// if x

// 3. else if checks multiple conditions

// 4. Rust runs only first true branch

// 5. if is an expression
// so it can return value

// Example:
// let n = if true { 5 } else { 6 };

// 6. Both if and else must return same type

// Wrong:
// if true { 5 } else { "six" }

// 7. Use match later for cleaner multi-branch logic
