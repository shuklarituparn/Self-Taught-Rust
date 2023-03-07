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

    //if condition is true then 5 or else 6

    println!("The value of number is: {number}");
}
fn infinite_loop(){
    loop { //tells to loop again and again unless told to stop
        println!("again!");
        break; //using break to get out of the infinite loop
    }

}
fn returning_value_from_loops(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //This loop runs, and the counter value gets from 0 to 10
    //As soon as it reaches 10 the program breaks the loop and returns the value of the counter*2
}
fn loops_inside_loops(){

    let mut count = 0;
    'counting_up: loop { //we can use label to a loop like here and use it later with break.
        println!("count = {count}");
        let mut remaining = 10;

        loop { //inside loop that counts in reverse and checks the condition below.
            println!("remaining = {remaining}");
            if remaining == 9 { //exits the inside loop after reaching "9" and increases counter
                break;
            }
            if count == 2 {
                break 'counting_up; //exiting the outside loop as soon as the count reaches 2
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
fn while_loop(){

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //While a condition evaluates to true, the code runs; otherwise, it exits the loop.
}
fn for_loop(){
    let a :[i32;5] = [10, 20, 30, 40, 50]; //5 signed integers of 32 bit
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

}
fn for_loop_in_a_collection(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_reverse(){
    for number in (1..4).rev() { //reversing from 4 to 1
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main(){
    if_statements_true();
    if_statements_false();
    else_if();
    if_in_a_let_statement();
    infinite_loop();
    returning_value_from_loops();
    loops_inside_loops();
    while_loop();
    for_loop();
    for_loop_in_a_collection();
    loop_reverse();

}
