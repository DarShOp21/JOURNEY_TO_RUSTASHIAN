fn main() {
    //Creating a new string
    let str = String::new();

    println!("{str}");

    let data = "Initial String";
    let str2 = data.to_string();
    println!("{str2}");

    // The method also works on a literal directly:
    let str3 = "New Initial String".to_string().to_uppercase();
    println!("{str3}");

    // String::from to create a String
    println!("\n\n=>String::from to create a String");
    let str4 = String::from("Hello");
    println!("{str4}");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Updating a String
    
    //Appending with push_str or push
    let mut str5 = String::from("foo");
    str5.push_str("bar");
    println!("{str5}");

    let mut str5 = String::from("foo");
    let s2 = "bar";
    str5.push_str(s2);
    println!("s2 is {s2}");

    //using 'push' -> it takes single character as a parameter
    let mut str6 = String::from("lo");
    str6.push('l');
    println!("{str6}");


    // Concatenating with + or format!
    println!("\n=>Concatenating with + or format!");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1+&s2;
    //  The + operator uses the add method, whose signature looks something like this:
    //
    //
    // fn add(self, s: &str) -> String {


    println!("{s3}");
    // println!("{s1}");    //ERROR WILL OCCUR BECAUSE s1 IS MOVED TO s3


    //CONCATENATING MULTIPLE STRING VALUES
    let s1 = String::from("TIC");
    let s2 = String::from("TAC");
    let s3 = String::from("TOE");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    //Alternative way to conat using format!()
    let s1 = String::from("TIC");
    let s2 = String::from("TAC");
    let s3 = String::from("TOE");

    let s = format!("{s1}-{s2}-{s3}");
    // it returns a String with the contents
    // format! macro uses references so that this call doesn’t take ownership of any of its parameters.

    println!("{s}");
    println!("{s1}");


    //TRYING INDEXING IN STRINGS
    let s1 = String::from("HELLO");
    // let h = s1[0];      //ERROR ->  the type `str` cannot be indexed by `{integer}`
    //indexing is not allowed because it supports multiple language support and each language has
    //different rules to store the VALUES
    // E.g => 1. HELLO -> [H , E , L , L , O]
    // E.g => 2. नमस्ते -> ['न', 'म', 'स', '्', 'त', 'े']
    //
    // IN Cyrillic letter each letter requires 2 bytes to store a single scalar value 

    //TO SOLVE THIS ISSUE WE CAN USE SLICING
    //SLICING IN STRINGS
    
    let hello = "Здравствуйте";
    let s = &hello[0..2]; // З
    println!("{s}");
    let s = &hello[0..4]; // Зд 
    println!("{s}");



    //ITERATING OVER STRING 
    println!("\n=>ITERATING OVER STRING ");

    //iterating over each char of Зд
    println!("->iterating over each char of Зд");
    for c in "Зд".chars(){
        println!("{c}");
    }

    //iterating over each byte of Зд
    println!("->iterating over each byte of Зд");
    for byte in "Зд".bytes(){
        println!("{byte}");
    }
    //Be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

    //CHECKING OF SUB_STRING IN STRING
    let str = "Hello";
    let bool = str.contains("He");
    
    match bool {
        true => println!("PRESENT"),
        _ => println!("NOT PRESENT")
    }

    //REPLACE 
    let new_str = str.replace("o","oz");
    println!("{new_str}");
}
