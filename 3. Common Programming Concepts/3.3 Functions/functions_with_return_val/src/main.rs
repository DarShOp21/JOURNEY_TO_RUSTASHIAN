// Functions with Return Values in Rust
//
// A function can send a value back to the caller.
//
// Syntax:
// fn function_name() -> datatype {
//      value
// }
//
// The last expression (without semicolon)
// becomes the return value.


// =====================================
// Function returning integer
// =====================================
fn five() -> i32 {
    5
    // Same as: return 5;
}


// =====================================
// Function taking parameter and returning value
// =====================================
fn plus_one(x: i32) -> i32 {
    x + 1
    // Returns x + 1
}


// =====================================
// Function using explicit return keyword
// =====================================
fn multiply_by_two(n: i32) -> i32 {
    return n * 2;
}


// =====================================
// Function returning string slice
// =====================================
fn greet() -> &'static str {
    "Hello from Rust"
}


// =====================================
// MAIN FUNCTION
// =====================================
fn main() {
    // Calling five()
    let f = five();

    println!("Value stored in f = {f}");

    // Direct function call inside println!
    println!("five() returns = {}", five());

    // Calling plus_one()
    println!("plus_one(10) = {}", plus_one(10));

    // Calling explicit return function
    println!("multiply_by_two(7) = {}", multiply_by_two(7));

    // Returning string
    println!("{}", greet());
}


// =====================================
// Important Wrong Example
// =====================================

// fn plus_one(x: i32) -> i32 {
//     x + 1;     // WRONG
// }
//
// Because semicolon converts expression into statement.
// Statement returns () not i32.


// =====================================
// Notes for Revision
// =====================================

// 1. Return type written after ->
// fn add() -> i32

// 2. Last expression without semicolon returns value

// Example:
// fn five() -> i32 {
//     5
// }

// 3. You can also use return keyword

// Example:
// return 5;

// 4. Semicolon stops returning value

// 5. () means unit type (empty return)

// 6. If no return type written,
// function returns ()

// Example:
// fn hello() {
//     println!("Hi");
// }

// 7. Rust prefers implicit return
// using last expression.
