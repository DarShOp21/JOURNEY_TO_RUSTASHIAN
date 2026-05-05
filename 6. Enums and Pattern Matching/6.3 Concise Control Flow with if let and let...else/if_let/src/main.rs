#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_from(&self) -> u32{
        match self {
            UsState::Alaska => 1819,
            UsState::Alabama => 1959
        }
    }
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {

    let config_max = Some(3u8);

    match config_max{
        Some(max) => println!("The maximum value is {max}"),
        _ => ()
    }

    //The below is the alternative for the same match
    //if let
    if let Some(max) = config_max{
        println!("The maximum value is {max}")
    }


    // let coin = Coin::Penny;
    let coin = Coin::Quarter(UsState::Alabama);
    

    let mut count = 0;
    if let Coin::Quarter(state) = coin{
        println!("State quarter from {state:?}");
    }else{
        count += 1;
    }

    println!("{count}");


    println!("Year of coins");

    let year = UsState::Alabama.existed_from();

    println!("Alabama is from {year}");
}

