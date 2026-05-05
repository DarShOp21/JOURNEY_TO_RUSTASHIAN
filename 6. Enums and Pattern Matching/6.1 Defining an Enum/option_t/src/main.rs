fn main() {
    let some_number = Some(5);
    let some_char = Some('a');

    let absent_num : Option<i32> = None;

    println!("NUMBER -> {:?}  CHAR -> {:?} ABSENT -> {:?}",some_number , some_char , absent_num);

    //adding a i8 with Option<i8>
    let x : i8 = 5;
    let y : Option<i8> = Some(10);

    let sum = x+y;  //error will occur 

    println!("{}",sum)
}
