
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_result = File::open("hello.txt");
    
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
           ErrorKind::NotFound => match File::create("hello.txt") {
               Ok(file) => file,
               Err(e) => panic!("Problem creating the file : {e:?}")
           },
           _ => {
               panic!("Problem opening the file : {error:?}");
           }
        },      
    };

    println!("{file:?}");

    //ALTERNATIVE TO USING MATCH WITH RESULT<T,E> unwrap_or_else METHOD

    let greeting_file = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound{
            //create the file
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file : {error:?}");
            })       
        }else{
            panic!("Problem opening the file : {error:?}");
        }
    });

    println!("{greeting_file:?}");


    //SHORTCUT FOR PANIC ON ERROR 
    //-> using .unwrap()
    // let greeting_file = File::open("hello3.txt").unwrap();
    // println!("{greeting_file:?}");

    //-> using .expect()
    let greeting_file = File::open("hello3.txt")
        .expect("hello3.txt should be included in the directory");
    println!("{greeting_file:?}");
        
}
