#[derive(Debug)]
struct User {
        active : bool,
        username : String,
        email : String,
        sign_in_count : u64
}   

fn main() { 
     let user1 = User{
        active:true,
        email:String::from("darshanayare43@gmail.com"),
        username:String::from("darshh_rustacean"),
        sign_in_count:1
    };

    println!("The email of {} is {}",user1.username , user1.email);

    //mutable struct 
    let mut user2 = User{
        active : true,
        email : String::from("nareshay7982@gmail.com"),
        username : String::from("naresh_ayare"),
        sign_in_count : 1
    };

    println!("The email of {} is {}",user2.username,user2.email);
    println!("Changing the email of {} to nareshay79@gmail.com",user2.username);

    user2.email = String::from("nareshay79@gmail.com");
    println!("The email of {} is {}",user2.username,user2.email);

        
    //Creating user using a function
    let user3_email : String = String::from("user3@gmail.com");
    let user3_username : String = String::from("userrr3");
    let user3 : User = create_user(user3_email, user3_username);
    println!("{}",user3.email);
    println!("USER3 -> {:?}",user3);

    //create a instance of type User with some same values of user3
    let user4 = User{
        active : user3.active,
        username : user3.username,
        email : String::from("user4@gmail.com"),
        sign_in_count : user3.sign_in_count
    };
    println!("USER4 -> {:?}",user4);

    //create a instance of type User with some same values of user2 using update syntax (..)
    let user5 = User{
        email : String::from("user5@gmail.com"),
        ..user2
    };
    println!("USER5 -> {:?}",user5);
    // println!("USER2 -> {:?}",user2)  error will occur
}   

fn create_user(email : String , username : String)->User{
    User{
        email ,
        username ,
        active : true,
        sign_in_count : 1
    }
}
