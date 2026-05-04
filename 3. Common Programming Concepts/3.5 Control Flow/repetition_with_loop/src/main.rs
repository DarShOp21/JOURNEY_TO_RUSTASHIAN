// Repetition with Loops in Rust
//
// Rust provides 3 main loops:
//
// 1. loop   -> infinite loop until break
// 2. while  -> runs while condition is true
// 3. for    -> iterate collections / ranges

fn main() {
    // =====================================
    // 1. Basic loop with break value
    // =====================================

    println!("========== loop ==========\n");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns 20
        }
    };

    println!("The result is {result}");



    // =====================================
    // 2. Loop Labels (Nested Loops)
    // =====================================

    println!("\n========== Loop Labels ==========\n");

    let mut count = 0;

    'counting_loop: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break; // exits inner loop only
            }

            if count == 2 {
                break 'counting_loop; // exits outer loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");



    // =====================================
    // 3. while loop
    // =====================================

    println!("\n========== while ==========\n");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!");



    // =====================================
    // 4. for loop with array
    // =====================================

    println!("\n========== for with array ==========\n");

    let a = [10, 20, 30, 40, 50];

    println!("=> using while loop");

    let mut index = 0;

    while index < a.len() {
        println!("The value is {}", a[index]);
        index += 1;
    }

    println!("=> using for loop");

    for val in a {
        println!("The value is {val}");
    }



    // =====================================
    // 5. for loop with range
    // =====================================

    println!("\n========== Range Loop ==========\n");

    println!("=> using range and rev");

    for number in (1..5).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!");
}



// =====================================
// Notes for Revision
// =====================================

// -------------------------------------
// loop
// -------------------------------------
// Runs forever until break.
//
// loop {
//     break;
// }

// break can return value:
//
// let x = loop {
//     break 5;
// };


// -------------------------------------
// continue
// -------------------------------------
// Skips current iteration
//
// loop {
//     continue;
// }


// -------------------------------------
// Labels
// -------------------------------------
// Useful in nested loops.
//
// 'outer: loop {
//     loop {
//         break 'outer;
//     }
// }


// -------------------------------------
// while
// -------------------------------------
// Runs while condition is true.
//
// while x > 0 {
//     x -= 1;
// }


// -------------------------------------
// for
// -------------------------------------
// Best for arrays, vectors, ranges.
//
// for item in arr {
// }

// Safer than while indexing.


// -------------------------------------
// Ranges
// -------------------------------------

// 1..5  => 1 2 3 4
// 1..=5 => 1 2 3 4 5

// rev() reverses range


// -------------------------------------
// Recommended Usage
// -------------------------------------

// loop   -> unknown repetitions
// while  -> condition based
// for    -> collections / counting
