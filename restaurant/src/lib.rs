mod front_of_house; // we need to do this only once in the module tree.
// After that the compiler knows the file is part of the project and how to get it

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
