use std::clone;

fn main() {
    //Ownership rules 
    //Each value in Rust has an owner.
    //There can only be one owner at a time.
    //When the owner goes out of scope, the value will be dropped.

    //variable scope
    let s = "hello";
    {
        println!("{s}");
    }
    println!("{s}");


    //The String Type
    let mut st = String::from("hello");
    st.push_str(", world");

    println!("{st}");

    //Variables and Data Interacting with Move
    let x = 6;
    let y = x;

    println!("{x}  {y}");

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{s1}");   => Will cause an error
    

    //Scope and Assignment
    let mut s = String::from("Hello");
    s = String::from("HELLO");

    println!("{s}");


    //Variables and Data Interacting with Clone
    let first = String::from("FIRST");
    let new_first = first.clone();

    println!("{first}  {new_first}");
}
