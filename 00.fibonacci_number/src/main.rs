use std::io;

fn fibonacci(number: i32) -> i32{   //arrow and the object next to it is the return value
    if number ==0 || number ==1{
       return number;
    }
    else{
        fibonacci(number - 1) + fibonacci(number - 2)
    }
}

fn main(){

    println!("Enter the term of the fibonacci sequence you want to find: ");
    let mut user_input = String::new();

    io::stdin() //handle to standard input stream of process
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let my_int = user_input.trim().parse::<i32>().unwrap();

    println!("fibonacci number is: {}", fibonacci(my_int));
}
/*
BufRead is a type of Reader which has an internal buffer, allowing it to perform extra ways of reading.

For example, reading line-by-line is inefficient without using a buffer, so if you want to read by
line, you’ll need BufRead, which includes a read_line method as well as a lines iterator
 */

/*
Note that you cannot use the ? operator in functions that do not return a Result<T, E>.
Instead, you can call .unwrap() or match on the return value to catch any possible errors:
 */

//parse converts the string into int.
//trim removes all the whitespaces