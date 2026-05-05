// use std::cmp::Ordering;
// use std::io;
//      |
//      V
//alternative and preferred way to import things
use std::{cmp::Ordering, io};



// use std::io;
// use std::io::Write;
//      |
//      V
//alternative and preferred way to import things
use std::io::{self , Write};


//Glob operator
//brings all public items defined in a path into scope
use std::collections::*;    //This use statement brings all public items defined in std::collections into the current scope.
fn main() {
    println!("Hello, world!");
}
