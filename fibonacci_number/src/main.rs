use std::io;

fn fibonacci(number: i32) -> i32{
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

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let my_int = user_input.trim().parse::<i32>().unwrap();

    println!("fibonacci number is: {}", fibonacci(my_int));
}
