fn if_statements_true() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
fn if_statements_false() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

/*
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

If we type the above code we'll get an error as the if function expects a true or false value but
it gets the value of 3 instead.
 */

fn else_if(){

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    /*
    we got the result "divisible by 3" and not divisible by 2 because rust only executes the block
    for the first true condition, and as soon as it finds one it doesn't even check the rest.
    */
}

fn if_in_a_let_statement(){

    let condition = true;
    let number = if condition { 5 } else { 6 };

    //if condition is true than 5 or else 6

    println!("The value of number is: {number}");
}
fn main(){
    if_statements_true();
    if_statements_false();
    else_if();
    if_in_a_let_statement();
}
