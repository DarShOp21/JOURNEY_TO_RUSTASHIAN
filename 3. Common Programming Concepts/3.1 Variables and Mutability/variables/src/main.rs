// fn main() {
//     let x = 5;
//     println!("The value of x is {x}");
//
//     x = 6;
//     println!("The value of x is {x}");
// }
//



// fn main() {
//     let mut x = 5;
//     println!("The value of x is {x}");
//
//     x = 6;
//     println!("The value of x is {x}");
// }
//


//SHADOWING
// fn main(){
//     let x = 5;
//     println!("The value of x is {x}");
//
//     let x = x+1;
//     println!("The value of x is {x}");
//
//     {
//         let x = 10;
//         println!("The value of x is {x}"); 
//     }
//
//     println!("The value of x is {x}");
// }


// fn main(){
//     let spaces = "  ";
//     println!("{spaces}");
//
//     let spaces = spaces.len();
//     println!("{spaces}");
// }


fn main(){
    let mut spaces = "  ";

    spaces = spaces.len();
}
