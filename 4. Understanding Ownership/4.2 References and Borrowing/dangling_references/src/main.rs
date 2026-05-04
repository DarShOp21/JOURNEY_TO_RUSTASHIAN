fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s = String::from("hello");

    &s
}
// Here, s goes out of scope and is dropped, so its memory goes away.
// Danger!
