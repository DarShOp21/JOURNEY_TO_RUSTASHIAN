fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{}",hello);

    
    //if you want to start at index 0, you can drop the value before the two periods
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    //both slice1 and slice2 are same
    

    // if your slice includes the last byte of the String, you can drop the trailing number
    let len = s.len();
    let slice3 = &s[3..len];
    let slice4 = &s[3..];
    //both slice3 and slice4 are same
    

    //can also drop both values to take a slice of the entire string
    let slice5 = &s[0..len];
    let slice6 = &s[..];
    //both slice5 and slice6 are same


    //PERFORMING FIRST WORD PROBLEM USING SLICE 
    let first = first_word(&s);
    println!("FIRST WORD OF {s} is -> {first}");




    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s:&str)->&str{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return  &s[0..i];
        }
    }

    &s[..]
}
