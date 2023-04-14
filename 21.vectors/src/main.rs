fn main() {
    let _v: Vec<i32> = Vec::new(); //to create a new vector. we have to give type annotation <i32> to tell Rust that kind of the data we want it to store

    let _v = vec![1, 2, 3]; //if we wanna pre-intialize the vector we have to use the ! sign

    let mut m = Vec::new(); //creating a new mutable variable m of type vector. V in caps

    m.push(1);
    m.push(3);
    m.push(7); //updating our vector

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; //using the index method to get the data
    println!("The third element is {third}"); //printing the value of the vector

    let third: Option<&i32> = v.get(2); //here if we remove <&i32> it won't run  as it wants the info about the type
    match third {
        //error handling
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    iterating_on_a_vector(); //calling the iterating on vector function here
    changing_vec_values();
}

/*
Using & and [] gives us a reference to the element at the index value. When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.

The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements. As an example, let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique, as shown in Listing 8-5.
 */

//ITERATING IN A VECTOR

fn iterating_on_a_vector() {
    let v = vec![5, 7, 8, 9, 10, 24];

    for elements in &v {
        println!("{elements}"); //println! macro prints on a new line
    }
    for elements in &v {
        print!(": {elements}"); //print! macro prints on the same line
    }
}

fn changing_vec_values(){

    //we can iterate using the loop over a vector to 

    let mut m = vec![5, 7, 8, 9, 10, 24];

    for elements1 in &mut m{  //without &here it says that we iterating on addresses
        *elements1+= 50;  //had to use * to dereference the elements before adding 50
    }
    for elements1 in &m {
        print!(":: {elements1}");

        //if we instead did "{{elements}}" it would print it 6 times
    }
}
