// Scalar Types in Rust
// This program demonstrates Numeric, Boolean, and Character types
// All examples are integrated into a single main() function

fn main() {
    // ==============================
    // Numeric Operations
    // ==============================
    println!("--- Numeric Operations ---");

    let sum = 5 + 6;
    println!("Sum: {sum}");

    let diff = 10 - 4;
    println!("Difference: {diff}");

    let prod = 9 * 6;
    println!("Product: {prod}");

    let quotient = 56.7 / 32.2; // floating-point division
    println!("Quotient: {quotient}");

    let truncated = -5 / 2; // integer division (decimal removed)
    println!("Truncated Division: {truncated}");

    let remainder = 45 % 2; // modulus gives remainder
    println!("Remainder: {remainder}");

    // ==============================
    // Boolean Type
    // ==============================
    println!("\n--- Boolean Type ---");

    let f: bool = false;
    println!("Value of f: {f}");
    println!("Negation of f: {}", !f);

    // ==============================
    // Character Type
    // ==============================
    println!("\n--- Character Type ---");

    let c = 'X';
    let z: char = 'z';
    let heart_eyed_cat: char = '😻';

    println!("Character c: {c}");
    println!("Character z: {z}");
    println!("Emoji char: {heart_eyed_cat}");
}
