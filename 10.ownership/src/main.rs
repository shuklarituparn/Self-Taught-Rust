/*

In order to take mutable input of string rust uses STRING type and uses the "from" function


 */

fn main(){

let s = String::from("hello");
println!("{}", s);

string1();
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