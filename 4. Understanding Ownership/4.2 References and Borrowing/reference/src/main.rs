fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}");

    //Mutable References
    change(&mut s1);

    println!("{s1}");

    // let r1 = &mut s1;
    // let r2 = &mut s1;
    //
    // println!("{r1} {r2}");  //error will occur because a value cant have multiple mutable reference
                            //at a time
    
    //solution
    {
        let r1 = &mut s1;
        println!("{r1} from r1");
    }//r1 goes out of scope after this
    let r2 = &mut s1;
    println!("{r2} from r2");



    // let ref1 = &s1;
    // let ref2 = &s1;
    // let ref3 = &mut s1;     //error will occur immutable reference don’t expect the value to suddenly change out from under them
    //
    // println!("{ref1} {ref2} {ref3}");



    let ref1 = &s1;
    let ref2 = &s1;
    println!("{} {}",*ref1,*ref2);
    // Variables r1 and r2 will not be used after this point.

    let ref3 = &mut s1;     //no problem
    println!("{ref3}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(s:&mut String){
    s.push_str(", world");
}
