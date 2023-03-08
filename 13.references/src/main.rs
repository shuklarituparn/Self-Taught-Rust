//references and borrowing 

fn main() {
    let s1 = String::from("hello");

    let mut s2 = String::from("hello"); //for mutable reference need to declare variable as mutable

    let len = calculate_length(&s1); //Here we don't need to pass both of the argument to
                                                // the function. We just pass the address

    println!("The length of '{}' is {}.", s1, len);

    mutable_reference_change(&mut s2);

    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn mutable_reference_change(some_string: &mut String){

    some_string.push_str(", world");

}

/*

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

//cannot borrow a variable as mutable more than once

 The benefit of having this restriction is that Rust can prevent data races at compile time. 
 A data race is similar to a race condition and happens when these three behaviors occur:

    Two or more pointers access the same data at the same time.
    At least one of the pointers is being used to write to the data.
    There’s no mechanism being used to synchronize access to the data.


let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

//Can use curly braces to make an instance


 */


/*
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
//cannot borrow a variable as mutable and immutable at once

//We can get around that problem by using the immutable values before making it mutable

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem 
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); 
    // variables r1 and r2 will not be used after this point and hence no problem

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
 */

/*
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello"); //S comes into scope

    &s //reference to s
}    //s goes out of the scope and the memory is cleared 


//The solution to this problem is to return the string not the reference

fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

 */

/*

The Rules of References

Let’s recap what we’ve discussed about references:

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.




 */