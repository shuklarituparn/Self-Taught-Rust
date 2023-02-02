fn main() {
    let x=5;
    println!("The value of x is ; {x}");

    mutable_var(); //calling new function inside the main

    const_val();

    println!("Shadowing a variable");
    shadow_var();
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

//*************************************************CONSTANTS***************************************************//

//Constants are values that are bound to a name and are not allowed to change

//Not allowed to use mut with constants. They are always immutable.

//The last difference is that constants may be set only to a constant expression,
//not the result of a value that could only be computed at runtime.

//Rust convention for const is to use all caps with space in between

fn const_val(){

    const THREE_HOURS_IN_SECONDS: u32= 60*60*3;
    println!("Three hours in seconds is : {THREE_HOURS_IN_SECONDS}");
}

//******************************************Shadowing********************************************************//

//shadowing is the process of using  a variable again and again without redeclaration

fn shadow_var() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //Value of X after operations on it
    }

    println!("The value of x is: {x}"); //Value of X based on original declaration


}

//Shadowing is different from mut as by using let we perform some operations on the variable and after
//the operations are done the variable is immutable again.

/*Another difference b/w shadowing and mut is that by using let we can change the type of variable, while in
mutable form by using mut the variable type remains the same

let spaces="    "; //var of type string
let spaces= spaces.len();
 */
