#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
       
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    
}
/*
impl Rectangle {
    fn square(size: u32) -> Self {  //Returning a struct (or itself) in this case a rectangle
        Self {
            width: size,
            height: size,
        }
    }
}





 */

/* The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle*/
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



/*

 we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type.

 The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
 Methods must have a parameter named self of type Self for their first parameter

 */

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); //calling a method callled can hold to check if a rectangle can hold other
    

}