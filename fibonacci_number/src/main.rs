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


    println!("fibonacci numbers are: {}", fibonacci(11));
}
