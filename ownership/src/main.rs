// Rust doesn't have a garbage collector
// Rust doesn't permit allocation
//
// Rust uses ownership system
//
// Ownership is about managing value onto the heap
// values on the stack have fix size know at compile time
// values on the heap have size that we don't know at compile time
//
// 1. Each value in Rust has an owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


// NOTE : A reference scope starts from where it is introduced an continues through the last time
// that reference is used.


fn main() {
    // s is not valid here because it's not yet declared
    {
        let s = "hello"; // s is valid
    }
    // s is no longer valid, because the scope is over

    let mut s = String::from("hello"); // declare a mutable string that is stored in the heap
    s.push_str(", world"); // can change the value of the string


    let s1 = String::from("hello");
    let s2 = s1; // s2 will be a copy of the stack element of s1 (capacity, pointer to value first adress, len)
    // So the array storing "hello" will not be created two times
    // Heap data are copied that way
    //
    // This instruction will cause a move
    // A move mean that s1 will no longer be usable and s2 will have his value
    // This technic permit to avoid double free error (free s1 and s2 when out of scope)


    // if you want to deeply copy the heap data use the clone method
    let s3 = s2.clone(); // s2 will still be usable and s3 also

    // for primitive we don't need to do this because they are stored in the stack
    //
    // Also for types that implements the Copy trait, variables that use it do not move, but rather
    // are trivially copied, making them still valid after assignment to another variable.
    //
    //
    // Ruts won't let us annotate a type with Copy if the type, or any of its parts, has
    // implemented the Drop trait. Implementing both will result in compile-time error.
   
    
    // Passing argument to a function is the same as assignment
    let s = String::from("hello"); // s come into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                    // but i32 is Copy, so it's okay to still use x afterward
    
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we pass an adress as a parameter of the function

    let mut s = String::from("hello");
    correct_change(&mut s); // remark the mut to precise that we want a mutable reference
   
    let r1 = &mut s;
    let r2 = &mut s; // we cannot have to mutable reference to the same variable
    let r3 = &s; // we cannot also have a mutable reference and a mutable one at the same time
    //
    println!("{} and {}", r1, r2); // after using both mutable references we can use the immutable
    // one
    let r4 = &s;
}

fn takes_ownership(some_string: String){ // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32){ // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Returning value can also transfer ownership.

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize { // this function uses a reference as parameter so that
    // the ownership isn't borrow from the main scope
    s.len() 
} // here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.
// This action is called to borrow ownership

fn change(some_string: &String){
    some_string.push_str(", world"); // we cannot change something that we borrowed
}

fn correct_change(some_string: &mut String){
    some_string.push_str(", world"); // but we can do that if we use a mutable reference
}
