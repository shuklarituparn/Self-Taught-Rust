#[derive(Debug)]
enum UsState {
    Alabama, //enum takes a value of U.S. states 
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  //This is variant that includes another enum called USstate
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state); //the value state has a binding of Alaska and now it'll match it and print 
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska)); //calling that enum
}
/*

Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute 
code based on which pattern matches. 

With if the condition needs to be evaluated to  a boolean while with Match it can match to any value 
 */