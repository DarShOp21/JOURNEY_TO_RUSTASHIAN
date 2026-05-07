use std::{i32, vec};
use std::collections::HashMap;

fn main() {
   let v1 : Vec<i32> = vec![10,15,20,20,30,30,40,40,40];

   println!("MEDIAN -> {}",median(&v1));

   println!("MODE -> {}",mode(&v1));
}

fn median(vec : &Vec<i32>) -> i32{
    let length = vec.len();

    // vec[length/2];
    vec[length/2]
}

fn mode(vec : &Vec<i32>) -> i32{
    let mut map = HashMap::new();

    let mut max : (i32,i32) = (-1,-1);
    // let max :(i32 , i32) = (&vec[0] , 0); 
    for numb in vec{
        let count = map.entry(numb).or_insert(0);
        *count += 1;
        if *count > max.1{
            max = (*numb , *count)
        }
    }

    max.0
}
