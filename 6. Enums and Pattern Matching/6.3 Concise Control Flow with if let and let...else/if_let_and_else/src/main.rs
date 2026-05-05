use std::fmt::format;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState{
    fn existed_in(&self , year : u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
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


fn describe_state_quarter(coin : &Coin) -> Option<String>{
    if let Coin::Quarter(state) = coin{
        if state.existed_in(1900){
            Some(format!("{state:?} is preety old for America"))
        }else{
            Some(format!("{state:?} is relatively new"))
        }
    }else{
        None
    }
}

//alternative way to define fn describe_state_quarter
fn describe_state_quarter2(coin : &Coin) -> Option<String>{
    let Coin::Quarter(state) = coin else{
        return None;
    };

    if state.existed_in(1990){
        Some(format!("{state:?} is preety old , for America"))
    }else{
        Some(format!("{state:?} is relatively new"))
    }
}

fn main() {

    let coin1 = Coin::Dime;
    let coin2 = Coin::Quarter(UsState::Alaska);

    let desc_for_coin1 = describe_state_quarter(&coin1);
    let desc_for_coin2 = describe_state_quarter(&coin2);

    println!("Description for {coin1:?} -> {desc_for_coin1:?}");
    println!("Description for {coin2:?} -> {desc_for_coin2:?}");

    println!("\n USING ALTERNATIVE FUNCTION");

    let desc_for_coin1_alt = describe_state_quarter2(&coin1);
    let desc_for_coin2_alt = describe_state_quarter2(&coin2);

    println!("Description for {coin1:?} -> {desc_for_coin1_alt:?}");
    println!("Description for {coin2:?} -> {desc_for_coin2_alt:?}");
}
