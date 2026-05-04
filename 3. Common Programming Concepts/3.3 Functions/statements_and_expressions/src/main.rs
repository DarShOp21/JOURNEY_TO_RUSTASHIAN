// Statements and Expressions in Rust
//
// Rust is an expression-based language.
// That means many blocks of code produce values.
//
// -------------------------------------
// Statement:
// Performs an action but returns NO value.
//
// Examples:
// let x = 5;
// fn hello() {}
//
// -------------------------------------
// Expression:
// Evaluates to a value.
//
// Examples:
// 5 + 6
// x + 1
// function_call()
// if condition { ... } else { ... }
// { block_of_code }

fn main() {
    // =====================================
    // Invalid Example (Statements don't return value)
    // =====================================

    // let x = (let y = 6);   // ERROR
    // Because:
    // let y = 6; is a statement
    // Statements do NOT return values

    // =====================================
    // Block Expression Example
    // =====================================

    let y = {
        // This is a new block scope

        let x = 3; // statement

        // expression (no semicolon)
        // Returned from block
        x + 1
    };

    // y gets value 4
    println!("The value of y is: {y}");

    // =====================================
    // More Expression Examples
    // =====================================

    let sum = 5 + 6; // expression returns 11
    println!("Sum = {sum}");

    let square = {
        let n = 5;
        n * n
    };

    println!("Square = {square}");

    // if is also an expression
    let number = 10;

    let result = if number > 5 {
        "Greater"
    } else {
        "Smaller"
    };

    println!("Result = {result}");
}


// =====================================
// Notes for Revision
// =====================================

// 1. Statement = does work, returns nothing
// Example:
// let x = 5;

// 2. Expression = gives value
// Example:
// 5 + 2

// 3. Semicolon converts expression into statement

// Example:
// x + 1     => returns value
// x + 1;    => no return value

// 4. Block {} can return last expression

// Example:
// let a = {
//     let x = 3;
//     x + 1
// };

// a = 4

// 5. if / match / function calls are expressions in Rust

// 6. Rust heavily uses expressions,
// so understanding this concept is important.
