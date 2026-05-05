fn main() {
    let dice_roll : u8= 9;

    // match dice_roll{
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other)
    // }
    
    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(dice_roll) 
    }
}

fn add_fancy_hat(){
    println!("ADDED FANCY HAT!");
}

fn remove_fancy_hat(){
    println!("REMOVED FANCY HAT!");
}

fn move_player(moves : u8){
    println!("MOVED PLAYER BY {moves} moves");
}
