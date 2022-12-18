// scalar types : represents a single value
// Four primary scalar types : integers, floating-point, numbers, Booleans, characters

// -------------------------

// Integers Types
// Unsigned (u) | Signed (i)
// + place taken (8, 16, 32, 64, 128)
// Ex : i8 (8 bits signed integer) u128 (128 bits unsigned integer)
//
//
// Floats
// start with "f" + place taken
// Ex: f32 (32 bits float)
// Floats are always signed
//
//
// Characters
// From U+0000 to U+D7FF
// emoji are taken into account

// -------------------------

// Compound types
// Represented by tuples
// fixed lengths elements
//
// Ex: let tup: (i32, f64, u8) = (500, 6.4, 1);
//
// Compound element of signed 32 bits integer, 64 bits float, 8 bits unsigned integer
// Access elements : with "."
// Ex: tup.0 will return 500
//
// Another example:
// let (x, y, z) = tup;
// x will be 500, y will be 6.4, z will be 1


// -------------------------

// Array type
// arrays are fixed lengths elements
// Cannot handle multiple types elements
//
// Ex: let months = ["January", "February", "March", "April"];
//
// You can precise the lenght and the elements types
// Ex: let a: [i32; 5] = [1, 2, 3, 4, 5];
//
// Initialization with element that have the same value
// Ex: let a = [3; 5]
// a will have [3, 3, 3, 3, 3]
//
// Access element with index in square brackets
// Ex: a[2] will return 3, a[0] will return 1
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false; // boolean

    let c = 'z';
    let z: char = 'ℤ'; // character

    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup;
    let five_hundred = tup.0;

    let a = [1, 2, 3, 4, 5]; // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}
