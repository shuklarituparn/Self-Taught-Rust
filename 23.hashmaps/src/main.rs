fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //key is the strings and values is the number

    //we are tracking two teams blue and yellow with starting score of 10 and 50 respectively

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    /*
    Here, score will have the value that’s associated with the Blue team, and the result will be 10.
     The get method returns an Option<&V>; if there’s no value for that key in the hash map, get
     will return None. This program handles the Option by calling copied to get an Option<i32>
     rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an
     entry for the key.
     */

    for (key, value) in &scores {  //we can iterate over a key, value pair
        println!("{key}: {value}");
    }

    let mut map = HashMap::new();
    //map.insert(field_name, field_value); not found in this scope
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores) //print 25 as the older value has been overwritten


    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    //using the entry method to only add if the key doesn't have a value already

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    /*
     We use a hash map with the words as keys and increment the value to keep track of how many
     times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the
     value 0.
     */

    /*
    This code will print {"world": 2, "hello": 1, "wonderful": 1}.

   split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the
   value in text. The or_insert method returns a mutable reference (&mut V) to the value for the
   specified key. Here we store that mutable reference in the count variable, so in order to assign
    to that value, we must first dereference count using the asterisk (*). The mutable reference
    goes out of scope at the end of the for loop, so all of these changes are safe and allowed by
    the borrowing rules.
     */


}