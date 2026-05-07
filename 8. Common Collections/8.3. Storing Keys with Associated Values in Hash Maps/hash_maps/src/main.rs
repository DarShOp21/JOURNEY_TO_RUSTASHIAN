use std::collections::HashMap;



fn main(){
    let mut scores = HashMap::new();  

    scores.insert(String::from("BLUE"), 10);
    scores.insert(String::from("RED"), 20);

    println!("{scores:?}");

    //Accessing Values in a Hash Map
    println!("=>Accessing Values in a Hash Map");
    let team_name = String::from("BLUE");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score:?}");

    //Iterating over HashMap
    println!("\n=>Iterating over HashMap");
    for (key , value) in &scores{
        println!("{key} : {value}");
    }

    //Managing Ownership in Hash Maps
    println!("\n=>Managing Ownership in Hash Maps");
    let field_name = String::from("Fav color");
    let field_value = String::from("BLUE");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}");   //ERROR -> bcoz the field_name was moved during map.insert(field_name , field_value)


    //Updating a Hash Map
    println!("\n=>Updating a Hash Map");
    
    println!("->Overwriting a Value");

    let mut scores = HashMap::new();

    scores.insert(String::from("BLUE"), 10);
    println!("{scores:?}");

    scores.insert(String::from("BLUE"), 20);
    println!("{scores:?}");

    println!("->Adding a new Key and Value Only if a Key Isn't Present");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(20);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}"); //OP -> {"Blue": 10, "Red": 20}

    println!("->Updating a Value Based on the Old Value");
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
