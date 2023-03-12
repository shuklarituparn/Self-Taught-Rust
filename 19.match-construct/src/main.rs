#![allow(unused)]



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

    fn plus_one(x: Option<i32>) -> Option<i32> { //Matching with Option<T>
        match x {                                   //Match is exhaustive it should have a compare case of all possible values
            None => None,      //if the input is not a number doing nothing . Match should cover all test cases so the "NONE" is necessary here
            Some(i) => Some(i + 1),    //if the input is integer adding +1
        }
    }

    let five = Some(5);     
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),  //
        7 => remove_fancy_hat(),
        other => move_player(other),//catch-all condition to catch all the remaining cases. Should be in end as match goes one by one to all cases
        _ => (), //can also use "_"
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}




}
/*

Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute 
code based on which pattern matches. 

With if the condition needs to be evaluated to  a boolean while with Match it can match to any value 
 */

