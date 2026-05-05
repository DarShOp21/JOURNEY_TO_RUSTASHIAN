#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin : &Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("LUCKYY PENNY!!");
            1
        }
        Coin::Nickel => 5 ,
        Coin::Dime => 10 ,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        } ,
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);    

    let value = value_in_cents(&coin);
    println!("The value of the coin {:?} is {} cent/cents" , coin , value);
}



