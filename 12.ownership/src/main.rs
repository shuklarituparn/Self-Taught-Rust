/*

In order to take mutable input of string rust uses STRING type and uses the "from" function


 */

fn main(){
fn boxed();

let s = String::from("hello");
println!("{}", s);

string1();
string2();
string3();

let s3 = String::from("hello");

let (s3, len) = calculate_length(s3); //Actually rust can return 2 values.

println!("The length of '{}' is {}.", s3, len); 

takes_ownership(s);             // s's value moves into the function...
                                            // ... and so is no longer valid here

let x = 5;                      // x comes into scope

makes_copy(x);                     // x would move into the function,
                                                // but i32 is Copy, so it's okay to still
                                                // use x afterward

let s1 = gives_ownership();         // gives_ownership moves its return
                                                // value into s1
print!("{}",s1);


let s2 = String::from("hello");         // s2 comes into scope
println!("{}",s2);
        
let s3 = takes_and_gives_back(s2);  // s2 is moved into
println!("{}", s3);                                                    // takes_and_gives_back, which also
                                                    // moves its return value into s3
                                                    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
                                                    // happens. s1 goes out of scope and is dropped.
        
}                                               // Here, x goes out of scope, then s. But because s's value was moved, nothing
                                                // special happens.



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

Something similar happens with the 22.strings.

 */

 fn string2() {
    let s1 = String::from("Hey\n");
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

/********************************************************************/ /*Stack-Only Data: Copy****************************************** */// 

/*
fn main() {
    let x = 5; 
    let y = x;

    println!("x = {}, y = {}", x, y);
}

In this case we didn't have to call the copy function or the clone as the compiler knows the size of the int on the stack so it is easy to make a copy and we don't 
have to actually make a deep copy as it doesn't makes sense.

 */

/*
what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values 
can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

    All the integer types, such as u32.
    The Boolean type, bool, with values true and false.
    All the floating-point types, such as f64.
    The character type, char.
    Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

 */


fn takes_ownership(some_string: String) {       // some_string comes into scope. This function accepts a variable of type string_from
    println!("{}", some_string);                
}                                              // Here, some_string goes out of scope and `drop` is called. The backing
                                              // memory is freed.

fn makes_copy(some_integer: i32) {              // some_integer comes into scope
    println!("{}", some_integer);
}                                              // Here, some_integer goes out of scope. Nothing special happens.

//************************************************************************// Return Value and Scopes //**********************************************************/

//Returning values can also transfer ownership. 

   
fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // Takes a variable of the type string_from type and returns it.

                                                      

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn boxed(){
    let a = Box::new([0; 1_000_000]); //allocating the memory on the heap for a million objects
    let b = a; //allocating that array to b
    
}