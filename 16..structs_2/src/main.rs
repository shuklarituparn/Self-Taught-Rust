fn main() {
    let rect1 = (30, 50); //can use tuples to declare a variable

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(dimensions: (u32, u32)) -> u32 {  //the tuple argument to the function called area that takes a tuple and return an integer
    dimensions.0 * dimensions.1
}

#[derive(Debug)]  //Using this as rust doesn't know how to format struct for printing on the screen
struct Rectangle {
    width: u32,
    height: u32,
}

/*
dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro 
call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
 */

