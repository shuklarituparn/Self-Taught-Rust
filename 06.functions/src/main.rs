/*
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
*/

//*************************PARAMETERS IN RUST*****************************************//


fn parameters() {
    another_function(5);
}

fn another_function(x: i32) //necessary to indicate type
{
    println!("The value of x is: {x}"); //Parameters will be placed in curly braces
}


//***************************Multiple Parameters*************************************//


fn multiple_parameters() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}




//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value. Let’s look at some examples.

fn statement() {
    let y = 6;
    println!("Here Y={y} is a statement as it's not returning a value: ");
}


// let x = (let y = 6); error as can't assign let statement to another statement 

fn expression() {
    let y = {
        let x = 3;
        x + 1 //expressions that return a vlaue don't end in semicolon in rust
    };

    println!("The value of y is: {y}");
}
//the code after y is a block that evaluates to a fix value. 

fn five() -> i32 {
    5
}

fn return__() {
    let x = five(); //binding x to the value that the function five returned

    println!("The value of x is: {x}");
}

//We dont't name return values but we must declare their type after an arrow

//in rust return value is the final expression in the end of function

//can return early using return keyword

fn main(){

    parameters();
    multiple_parameters();
    statement();
    expression();
    return__();

}