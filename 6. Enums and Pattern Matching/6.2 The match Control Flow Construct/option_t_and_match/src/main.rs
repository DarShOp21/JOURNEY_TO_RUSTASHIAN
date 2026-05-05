fn plus_one( x : Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(i) => Some(i+1)
    }
}

fn option_value( x : Option<i32>) -> i32 {
   match x{
       None => 0,
       Some(i) => i
   } 
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{five:?}  {six:?}  {none:?}");

    let ten = option_value(Some(10));
    println!("{ten}");
}


//The arms’ patterns must cover all possibilities
fn plus_two(x : Option<i32>) -> Option<i32> {
    match x{
        Some(i) => Some(i+2),
    }  //Error will occur ->> `None` not covered  
}
