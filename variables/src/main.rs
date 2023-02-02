fn main() {
    let x=5;
    println!("The value of x is ; {x}");

    mutable_var(); //calling new function inside the main
}
//by default var in rust are mutable. They store only one value.

//to make them store many values just add mut

fn mutable_var(){

    let mut y=5;
    println!("Value of y is: {y}");

    y=6; //no need to use mut keyword twice
    println!("The value of Y is also this: {y}")
}

//according to rust convention 

//Structs get camel case. 
//Variables get snake case.
//Constants get all upper case.