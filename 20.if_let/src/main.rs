/*
The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest. 
 */


 #[derive(Debug)]
 enum UsState {
     Alabama,
     Alaska,
     // --snip--
 }
 
 enum Coin {
     Penny,
     Nickel,
     Dime,
     Quarter(UsState),
 }
 
 fn main() {
     let coin = Coin::Penny;
     let mut count = 0;
     match coin {              //Can use match that goes over each test cases from one by one and matches them
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
     if let Coin::Quarter(state) = coin {     //or we can use if let that will make the code execute only if the condition we need gets true
         println!("State quarter from {:?}!", state);
     } else {
         count += 1;
     }
 }