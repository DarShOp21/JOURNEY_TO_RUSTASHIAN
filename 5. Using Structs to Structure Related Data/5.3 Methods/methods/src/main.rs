#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.length*self.width
    }

    fn width(&self)->u32{
        self.width
    }

    fn can_hold(&self,other_rect : &Rectangle)->bool{
        self.length > other_rect.length && self.width > other_rect.width
    }

    fn make_square(&self , size : u32)->Self{
        Self { 
            length: size, 
            width: size 
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        length:10,
        width:35
    };

    //area
    println!("The area of the rectangle is {}",rect1.area());

    //width
    println!("The width of the rectangle is {}",rect1.width());

    //methods with more parameters
    let rect2 = Rectangle{
        length:8,
        width:30
    };

    if rect1.can_hold(&rect2) {
        println!("RECT1 can hold RECT2");
    }else{
        println!("RECT1 cant hold RECT2");
    }

    //making rectangle to square
    println!("{:?}",rect2.make_square(10));
    println!("{rect2:?}");
}
