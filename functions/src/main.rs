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

/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) //necessary to indicate type
{
    println!("The value of x is: {x}"); //Parameters will be placed in curly braces
}
*/

//***************************Multiple Parameters*************************************//

fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


