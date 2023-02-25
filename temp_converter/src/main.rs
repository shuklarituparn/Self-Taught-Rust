use std::io;

fn main(){
    println!("Welcome to our program that converts temp: \n");
    let mut user_choice = String::new();
    let mut user_input= String::new();

    println!("Press 1 to convert fahr. to celsius: \n");
    println!("Press 2 to convert celsius. to fahr.: \n");
    io::stdin() //handle to standard input stream of process
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    let my_int = user_choice.trim().parse::<i32>().unwrap();


    //println!("Press 2 to convert celsius. to fahr.: ");

    if my_int==1 {
        println!("You chose to convert fahr. to celsius: \n");
        println!("Enter temperature in fahrenheit: \n");
        io::stdin() //handle to standard input stream of process
            .read_line(&mut user_input)
            .expect("Failed to read line");
        println!("The temp in celsius is: {}",fahr_to_celsius(user_input));
    }
    else if my_int ==2{
        println!("You chose to convert celsius to fahr. : \n");
        println!("Enter temperature in celsius: \n");
        io::stdin() //handle to standard input stream of process
            .read_line(&mut user_input)
            .expect("Failed to read line");
        println!("The temp in fahr is: {}",celsius_to_fahr(user_input));
    }
    else {
        println!("Wrong input, terminating the program:" );
        std::process::exit(1)
    }

}

fn fahr_to_celsius(user_input:String) -> f32{
    let mut temp = user_input.trim().parse::<f32>().unwrap();
    temp= (5.0/9.0)*(temp-32.0);
    return temp;
}
fn celsius_to_fahr(user_input:String) -> f32{
    let mut temp = user_input.trim().parse::<f32>().unwrap();
    temp= (9.0/5.0*temp)+32.0;
    return temp;
}