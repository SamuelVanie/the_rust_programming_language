/* fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


// Refactoring the code with tuples instead because the two parameters of area are related


fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
 */

// because tupes variables aren't named it's difficult to figure their meaning and we can mess up
// with passing values.
// A way to solve this is struct

#[derive(Debug)] // This is anotation which permits to use the debug format in println!
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );


    println!("rect1 is {}", rect1);  // This cannot be done because Rectangle doesn't implement the
    // Display trait

    println!("rect is {:?}", rect1); // This {:?} call the debug output format but the structure
    // have to implement it
    // {:#?} permits to use the debug on more larger structures,
    // The output format is different


    // You can use the !dbg to print values to the stderr
    // This function takes the ownership of the value passed to 
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // takes the ownership of scale but give the result back to width
        height: 50,
    }
    dbg!(&rect2); // pass a immutable reference because we don't want dbg! to take the ownership of
// rect2
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

