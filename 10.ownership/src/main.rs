/*

In order to take mutable input of string rust uses STRING type and uses the "from" function


 */

fn main(){

let s = String::from("hello");
println!("{}", s);

string1();
string2();
string3();

}

/*

 When a variable goes out of scope, Rust calls a special function for us. This function is called drop, 
and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
 */

fn string1(){

    let mut s = String::from("hello");
    
    s.push_str(", world!"); // push_str() appends a literal to a String
    
    println!("{}", s); // This will print `hello, world!`

    
    /*With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time,
    to hold the contents. This means:

    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with our String.

That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages*/
}


/*

Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust. 

fn main() {
    let x = 5;
    let y = x;
}

Here X is binded to 5 and then we are taking the value of x and binding it to y.

Something similar happens with the strings.

 */

 fn string2() {
    let s1 = String::from("Hey");
    let s2 = s1;
    print!("{}", s2); //we can see that the variable s2 holds the string that belonged to the variable s1

    //If we try to print the value of the variable s1 we'll get an error as it's value has been moved.
}

/*
When we move a string in rust it just makes the old string invalid and when the process ends it justs frees the second string 

In this way we don't have a memory leaks.

S1 holds three value the pointer to the first point in the string of the array, the second is the length of the array and the third one is the
capacity of the string.

When we do s1=s2 it just doesn't makes a copy which will be runtime costly but it just takes all the three value of the variable s1 and makes the variable s1 invalid

In this way we only have a single variable when the program ends

it's more effecient than making a copy and in reality we are just moving the data instead of making a copy

 */

//***************************************************************************// DEEP COPYING //************************************************************ */

//If we really do wanna deep copy in rust.

//Calling method clone helps us to do  a deep copy

fn string3() {
    let s1 = String::from("hemlo");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
