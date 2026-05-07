fn main() {
    let str = String::from("first");

    println!("{}",convert_str_to_piglatin(&str));
}

fn convert_str_to_piglatin(s : &str) -> String {

    let first_char = s.chars().next().unwrap();
    

    let vowels = ['a','e','i','o','u'];
    // println!("{first_char:?}");

    //if first is vowel , add hay at the end and return
    if vowels.contains(&first_char){
        return format!("{s}-hay");
    }

    //add "{first_char}ay" at the end and then remove the first char and return
    let remaining : String = s.chars().skip(1).collect();
    format!("{remaining}-{first_char}ay")
}
