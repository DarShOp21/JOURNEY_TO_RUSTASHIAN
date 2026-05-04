// Functions in Rust
// Functions help organize code into reusable blocks.
//
// Syntax:
// fn function_name(parameters) {
//      code
// }
//
// Rust naming convention:
// Use snake_case for function names and variables.

fn main() {
    // =====================================
    // MAIN FUNCTION
    // =====================================
    // main() is the entry point of every Rust program

    println!("Hello, world!");

    // Calling function with no parameters
    another_function();

    // Calling function with one parameter
    print_num(4);

    // Calling function with multiple parameters
    multiple_para(10, "darshan");

    // Calling function with different values
    multiple_para(21, "siddhesh");
}


// =====================================
// FUNCTION WITHOUT PARAMETERS
// =====================================
// This function takes no input and returns nothing

fn another_function() {
    println!("Another function");
}


// =====================================
// FUNCTION WITH ONE PARAMETER
// =====================================
// x is parameter of type i32
// Every parameter must have explicit type in Rust

fn print_num(x: i32) {
    println!("The value of x is {x}");
}


// =====================================
// FUNCTION WITH MULTIPLE PARAMETERS
// =====================================
// num -> integer
// name -> string slice (&str)

fn multiple_para(num: i32, name: &str) {
    println!("Number: {num}");
    println!("Name  : {name}");
}


// =====================================
// Notes for Revision
// =====================================

// 1. main() runs first in Rust program.

// 2. Function declaration syntax:
// fn name() { }

// 3. Function call syntax:
// name();

// 4. Parameter syntax:
// fn add(x: i32, y: i32)

// 5. Rust requires data types in parameters.

// 6. Multiple parameters separated by comma.

// 7. &str means string slice (borrowed string).

// 8. Functions can be declared before or after main().
// Rust only cares that function exists in scope.
