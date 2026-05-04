fn main() {
    let mut str = String::from("Darshan Naresh Ayare");

    println!("{}", first_word(&str));

    let word = first_word(&str);
    str.clear();

    println!("{word}");

}

fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}


