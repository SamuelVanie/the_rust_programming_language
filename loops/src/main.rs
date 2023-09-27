fn main() {
    // loop {
    //     println!("again!"); // infinite loop
    // }


    // You can return a value out of a loop
    // by adding it after the break expression you use to stop the loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    // You can add loop label to target a specific loop in continue and break expressions
    // break or continue without label will be applied to the innermost loop at that point
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");


    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // for loop
    // loop throw the element of an array
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // loop a certain number
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
