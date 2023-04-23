/*
Slices let you reference a contiguous sequence of elements in a collection rather than 
the whole collection. A slice is a kind of reference, so it does not have ownership.
*/

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
//fn second_word(s: &String) -> (usize, usize) {
fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    println!("{}\n", word);
    let word = first_word_slice(&my_string[..]);
    println!("{}\n", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_slice(&my_string);
    println!("{}\n", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word_slice(&my_string_literal[0..6]);
    println!("{}\n", word);
    let word = first_word_slice(&my_string_literal[..]);
    println!("{}\n", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);
    println!("{}\n", word);

}
//Converts a string slice to a byte slice. To convert the byte slice back into a
// string slice, use the from_utf8 function.

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


//String Slices

/*
A string slice is a reference to part of a String, and it looks like this:

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

let slice = &s[0..2];
let slice = &s[..2];

let slice = &s[3..len];
let slice = &s[3..];

let slice = &s[0..len];
let slice = &s[..]; //to take entire 22.strings

 here the string s will point to the first element.

it's length and the capacity is 11. which is total length of the string

However the world starts pointing to the 6 position in the string
 */