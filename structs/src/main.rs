// A struct or structure, is a custom data type that lets you package together and name multiple
// related values that make up a meaningful group.

// structs are like tuple and can handle multiples data
// But with structs you will be obliged to name your variables


// for instance's fields the entire instance must be mutable if you want to change it 
// Rust doesn't allow you to mark only certain fields as mutable

//  Rust has automatic referencing and dereferencing that is why we are not obliged to use a notation like (*object).method()
// or object->method()

// Methods in impl block are called associated functions in rust
// associated methods without &self as first parameter can be called only with Object::methods

struct User { // defining a struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Another kind of structs are tuple structs
// You can use these one if the name of the different fields doesn't add more sens to the struct
// itself
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // even if the both tuple structs have the same types, they are
// differents structures, you cannot switch between them.


// Here we go now with another one
// Unit-Like structs are structs that doesn't have any fields at all.
// They are similar to () which is the unit tuple
struct AlwaysEqual;

fn main() {
    // to create an instance of the struct you will have to git a value to each elements of this
    // struct
    
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };


    // It is possible to copy the value from an instance to create an another one
    let user2 = User {
        active: user1.active,
        username: user1.username, // because String doesn't implement the Copy trait this value is
        // moved and user1 is no longer available
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };


    // A shorthand for the previous declaration is ..user1
    let user2  = User {
        email: String::from("another@example.com"),
        ..user1 // This is called the struct update syntax
    };


    // you can access and change the values of a specific field by using the dot notation
    // instance.field
    user1.email = String::from("anothermail@example.com"); // This should work if user1 mail is
    // mutable

}

fn build_user(email: String, username: String) -> User {
    // function that create a new user instance with his parameters
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn shorthand_build_user(email: String, username: String) -> User{
    // Do not have to repeat the variable name
    // This is called field init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Defining methods
#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

impl Rectange { // function to calculate the area of a Rectangle
    // this is called an implementation block
    // everything within this block will be associated to the class
    fn area(&self) -> u32 { // methods names can be the same as the class name
	self.width * self.height
    }
}


