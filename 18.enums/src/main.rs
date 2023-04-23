#![allow(unused)]   // to allow unused code

enum IpAddrKind {     //Declaring  the enum 
    V4,                                     //types
    V6,
}


struct IpAddr {
    kind: IpAddrKind,  //can declare a struct that takes enum for a value
    address: String,   //address is string
}


fn main() {
    let four = IpAddrKind::V4;  //variable four is of type V4
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    enum IpAddr {
        V4(String),  //enums can take string as input. v4 is enum of type string
        V6(String),
        //V4(u8, u8, u8, u8),              //enums can also take tuples as input
    }

    let home = IpAddr::V4(String::from("127.0.0.1")); 

    let loopback = IpAddr::V6(String::from("::1"));

    //let home = IpAddr::V4(127, 0, 0, 1);  Passing a tuple to an enum

    enum Message { 
        Quit,        //variant with no data type associated with it
        Move { x: i32, y: i32 },   //variant with named data type like struct that accepts two integer
        Write(String),     //variant that accpepts 22.strings as input
        ChangeColor(i32, i32, i32),  //variant that accepts tuples as an argument
    }

    impl Message {       //can make imp on enums too
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));   //m is variable of enum called Message of type write that has a data "hello"
    m.call();  //calling method call on this 


    let home = crate::IpAddr {
        kind: IpAddrKind::V4,             
        address: String::from("127.0.0.1"),
    };

    let loopback = crate::IpAddr {   //importing the struct IpAddr using crate
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn route(ip_kind: IpAddrKind) {}


/*

enum Option<T> {
    None,
    Some(T),
}

<T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T
 makes the overall Option<T> type a different type.

    let some_number = Some(5);  //it makes the Option<T> of the type i32
    let some_char = Some('e');  //it makes the Option<T> of type char

    let absent_number: Option<i32> = None;
 */