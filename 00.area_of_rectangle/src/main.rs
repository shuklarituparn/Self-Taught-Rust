struct Rectangle{

    length: i32,
    breadth: i32, //Need to use  commas while declaring structs
}

fn area_of_rectangle(rectangle:&Rectangle)->i32{   //Need to pass the address of the struct in order to use them in the function
    //&Rectangle means we'll pass a reference of type struct called Rectangle
    let area= rectangle.length*rectangle.breadth;
    area
}



fn main() {
    let a = Rectangle{length: 1, breadth:20,};
    println!{"Area of the rectangle is: {}", area_of_rectangle(&a)}; //passing the address of the struct a into the function
}