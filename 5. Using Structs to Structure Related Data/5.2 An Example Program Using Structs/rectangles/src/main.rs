fn main() {
    let width1 = 20;
    let length1 = 35;

    println!("The area of the reactangle with length {length1} and width {width1} is {} square units",area(length1,width1));


    //Refactoring with Tuples
    let rect1 = (20,30);
    println!("The area of reactangle with length {} and width {} is using tuple is {} square units",rect1.0 , rect1.1 , area_using_tuple(rect1));

    //Refactoring with Structs
    let rect2 = Rectangle{
        length:30,
        width:10
    };
    println!("The area of rectangle with length {} and width {} using structs is {} square units", rect2.length , rect2.width,area_using_struct(rect2));
}

struct Rectangle{
    width : u32,
    length : u32
}

fn area(length:u32 , width:u32) -> u32{
    length*width
}

fn area_using_tuple(dimensions : (u32,u32))->u32{
    dimensions.0 * dimensions.1
}

fn area_using_struct(dimensions : Rectangle)->u32{
    dimensions.length*dimensions.width
}
