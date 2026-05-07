use std::vec;

fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3,4];

    //UPDATING A VECTOR
    let mut v3 = vec![10,20];
    v3.push(30);
    v3.push(40);
    // v3.push('a'); //error will occur
    


    //READING ELEMENTS OF VECTOR
    let third : &i32 = &v2[2];
    println!("Using &v2[2] , the third element is {third}");

    let third = v2.get(2);
    println!("Using v2.get(2) , the third element is {third:?}");  //OP -> Some(3)
    match third{
        Some(x) => println!("Using match in v2.get(2) , the third element is {x}"),
        None => println!("Their is no third element")
    }
    // println!("The third element is {third:?}");
    
    //READING VALUES OUTSIDE THE RANGE OF VECTOR
    println!("VECTOR(v3) -> {v3:?}");
    //READING VALUE AT INDEX 100
    // println!("{}",&v3[100]);    //ERROR -> index out of bounds: the len is 4 but the index is 100
    println!("{:?}",v3.get(100));  //OP -> None


    //Attempting to add an element to a vector while holding a reference to an item
    let mut v = vec![10,20,30,40,50];
    let first = &v[0];

    // v.push(60);      //ERROR WILL OCCUR

    println!("The first element is {first}");



    // ITERATING OVER VALUES IN VECTOR
    for i in &v{
        print!("{i} ");
    }
   
    println!("\n");
    //ITERATING OVER MUTABLE REFERENCE 
    for i in &mut v{
        *i *= 10;
        print!("{i} ");
    }
}
