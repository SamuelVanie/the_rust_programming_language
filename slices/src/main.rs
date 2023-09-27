// A string slice is a reference to part of a String
//
// [starting_index..ending_index] 
// starting_index : first position in the slice
// ending_index : one more than the last position in the slice.
//
// Not only string slices exists other atomic types can be used with slices also

fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];  // String slices examples
    let world = &s[6..11];

    let slice = &s[0..2]; // These two are equals
    let slice = &s[..2]; // We can omit the 0

    let len = s.len();
    
    let slice = &s[3..len]; // These are equivalent
    let slice = &s[3..]; // We can omit the ending if we want to go to the last index of the string
    
    let slice = &s[0..len];
    let slice = &s[..]; // whole string
    

    let my_string = String::from("hello world");

    let word = better_first_word(&my_string[0..6]);
    let word = better_first_word(&my_string[..]);
    let word = better_first_word(&my_string); // note that we can use now both string literal and
    // String type
}

fn first_word(s: &String) -> &str { // function that return the first word of in string
    // &str represent the return value for a slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn better_first_word(s: &str) -> &str { // that is a better signature because now we can take
    // String and str references as parameter

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
