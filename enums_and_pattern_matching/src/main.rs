// enums give you a way of saygin a value is one of a possible set of values.
// You can put any kind of data even Structs are allowed
// it's possible to define methods on enums like structs
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrK {
    V4(String), // define that the enum element have a type, it permits to avoid creating a struct
    V6(String),
}

enum IpAddr {
    V4(u8, u8, u8, u8), // the elements can be different, V4 ip addresses will always have 4 compenents
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // has named fields like a struct
    Write(String), // include a single string
    ChangeColor(i32, i32, i32), // includes three i32 values
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4; // we use namespace like syntax to declare an instance of Ipaddrkind

    let loopback = IpAddrK::V6(String::from("::1")); // create directely an instance with value for the enum

}

impl Message {
    fn call(&self) {
	// method body would be define here
    }
}

// The Option<T> enum is define in the standard library and represent
// the concept of a value being present or absent The variants of
// Option are Some and None Some for when the value if present None to
// represent no value

// We can extract value from an enum variant
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// To execute code depending on the value of the enum instance we use match block
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
	Coin::Quarter(state) => {
	    println!("State quarter from {:?}!", state);
	    25
	}
    }
}

// match should cover all possibilities
// For the other you can use any variable name that doesn't appear in the enum
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

// if you don't need the value of the other
// you use the _ as the name of the variable to signify that you don't want to use it
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

// Do nothing for the other value can be represented by tuple ()
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}


// If you want to match one pattern and ignore the rest you should be using
// if let control flow
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}

// This corresponds to 
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
