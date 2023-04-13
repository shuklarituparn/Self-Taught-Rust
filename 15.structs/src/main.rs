struct User {   //creating a struct 
    active: bool,   //data type that each variable will accept
    username: String,  
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); //can declare struct like tuples. 
struct Point(i32, i32, i32);

struct AlwaysEqual;// unit structs

struct User1 {
    active: bool,
    username: &str,  //normal for a struct to store references for data owned by some other variable
    email: &str, 
    sign_in_count: u64,
}


fn main() {
    let user1 = User{
        active: true,  //creating an instance of the string and giving it key:value
        username:String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,  //using commas instead of the semicolon in the struct 
    };

    user1.email= String::from("someone@example.com"); //using a dot nottation to acess something from a struct

    let black= Color {22,13,24};
    let origin= Point {0,0,0};
    
}


/*

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
to return a struct called user from a functiom

 */

/*

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,  //have same value as the original struct so it's not given 
        email,
        sign_in_count: 1,
    }
}


 */

/*

let user2 = User {
        active: user1.active,  //can use the data of some other struct in a struct 
        username: user1.username,  
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

 */

//If we wanna just say that the struct has a default values and we just wanna use the original data then we can use the double dots syntax

