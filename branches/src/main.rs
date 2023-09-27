fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // If are expression so we can add them in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    // numbers are expressions
    // block of code evaluate the last expresssion
    // So if and else statements should have the same type as result
    //
    // You shouldn't write :
    //
    // let number = if condition { 5 } else { "six" };


}
