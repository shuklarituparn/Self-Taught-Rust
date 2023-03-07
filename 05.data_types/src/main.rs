/*
Rust is a statically typed language, which means that it must know the types of all variables at compile time.
The compiler can usually infer what type we want to use based on the value and how we use it.
 */

/*
Scalar Types

A scalar type represents a single value.
Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
 */

/*
Integer Types in Rust
Length	Signed	Unsigned
8-bit	i8	      u8
16-bit	i16	      u16
32-bit	i32	      u32
64-bit	i64	      u64
128-bit	i128      u128
arch	isize     usize
 */

/*
To explicitly handle the possibility of overflow, you can use these families of methods provided by
the standard library for primitive numeric types:

    Wrap in all modes with the "wrapping_*" methods, such as "wrapping_add"
    Return the None value if there is overflow with the "checked_*" methods.
    Return the value and a boolean indicating whether there was overflow with the "overflowing_*" methods.
    Saturate at the value’s minimum or maximum values with the "saturating_*" methods.

 */

//**********************************************Floating point integers***************************************

//f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs,
// it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

fn floating_point(){

    let x= 2.0;  //default f64
    let y :f32 =3.0; //f32
    println!("The value of x and y is: {x}, {y}");

}

fn numeric_operations(){

    // addition
    let sum = 5 + 10;
    println!("The value of summing two values is: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of subtracting two values is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of multiplying two values is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    //let _truncated = -5 / 3; // Results in -1
    println!("The value of dividing two values is: {quotient}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of modulus of two values is: {remainder}");
}

fn bool_type(){
    println!("The boolean data type in Rust: ");
    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");
    //bool is 1 byte in rust
}
fn char_type(){
    println!("The char data type in Rust: ");
    let c = 'z';
    println!("char c: {c} ");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("char z: {z} ");
    let heart_eyed_cat = '😻';
    println!("char heart_eyed_cat: {heart_eyed_cat} ");

    //rust char type uses single quotes and string the double quotes. its four bytes
}
fn tuples(){

    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (a, y, z) = tup; //auto mapping the values of x,y,z to the tuple

    println!("The value of y is: {y}");
    println!("The value of a is: {a}");
    println!("The value of z is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // can access using period and index
    println!("The value of five_hundred is: {five_hundred}");

    let six_point_four = x.1;
    println!("The value of six_point_four is: {six_point_four}");
    let one = x.2;
    println!("The value of one is: {one}");
}
fn array(){

    let a = [1, 2, 3, 4, 5]; //five element of type i32
                                    //let a= [3;5] will create array of size 5 with each element =3

    let first = a[0];
    println!("First element of the array is: {first}");
    let second = a[1];
    println!("Second element of the array is: {second}");

}

fn main(){
    floating_point();
    numeric_operations();
    bool_type();
    char_type();
    tuples();
    array();
}