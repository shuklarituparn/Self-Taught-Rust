
/*
use std::io; //taking input and output from user so using io library from std
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //Using let to create a new variable called guess
                                   //mut means variable can take multiple values- mutable
                                   //by default variables in rust are immutable
                                   //string::new() creates mutable variable variable 
                                   //currently bound to empty instance of string 
                                
    io::stdin()      // if we didn't declare use std::io we'd have written std::io::stdin
        .read_line(&mut guess)  //calls read_line mehod on this and stores it on guess.
        .expect("Failed to read line"); // to handle error when the string is entered
                                        // gives two values "ok" or "err".
                                        // using expect so that our program crashes when we 
                                        //get an error. IMP TO DO ERROR HANDLING

    println!("You guessed: {guess}"); //curly braces hold value of var. between them
}
*/


//WRITING CODE TO MAKE USER GUESS RANDOM NUMBER
/*
use std::io;
use rand::Rng; //rng is a trait that defines method that random no. generator implements

fn main(){

    println!("Guess the number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100); 

   /*we call the rand::thread_rng function that gives us the particular 
   random number generator we’re going to use: one that is local to the current 
   thread of execution and is seeded by the operating system.*/

   /*we call the gen_range method on the random number generator. 
   This method is defined by the Rng trait that we brought into scope with the use 
   rand::Rng; statement. */

   /*The gen_range method takes a range expression as an argument and generates a 
   random number in the range. The kind of range expression we’re using here takes the 
   form start..=end and is inclusive on the lower and upper bounds, so we need to specify 
   1..=100 to request a number between 1 and 100.*/

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}*/

//To know which trait to use just use "cargo doc --open" to open documentations and click
// on the name of the crate


//To compare we use "use std::cmp::Ordering" 

//Ordering is an enum and has the value less,greater,equal

/*
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {  
             //Match is using refernce of secret number to compare it with guess
        Ordering::Less => println!("Too small!"), // use ordering type
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { //using infinite loop to give users infinite chances of guessing.
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //refers to original guess var with str
                                                    //trim will remove any whitespace
                                                //parse here converts string to number
            
            // parse return a result type and result is an enum with value ok or err
            //if parse sucessfully converts we get a number value as match would return the 
            //num value inside ok
            Ok(num) => num,
            Err(_) => continue, //no matter what error vlaue we get we ask user to enter 
                                //the number correctly again
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"), //for guessing 
            Ordering::Equal => {
                println!("You win!");
                break; //exiting the program after guessing correctly.
            }
        }
    }
}

//rust has inbuilt dtat types i32 is a 32 bit number.
//u32 is an unsigned 32 bit number
//i64 is a 64 bit number