use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}


/*
In order to include file go from left to right from the crate directory

pub mod garden tells the compiler to include the code it finds in the src/garden.rs

which is: pub mod vegetables

Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at :

                crate::garden::vegetables::Asparagus


Private vs public: 

Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.

The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to 

crate::garden::vegetables::Asparagus, 

you can create a shortcut with use 

crate::garden::vegetables::Asparagus; 

and from then on you only need to write Asparagus to make use of that type in the scope.

 */