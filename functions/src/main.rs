fn main() {
    println!("Hello, world!");

    let y = five();

    another_function(5, 'h');
    println!("value {y}")
}


fn another_function(value: i32, unit_label: char){
    println!("The measurement is {value}{unit_label}");
}


// no need return statement but can use them when it's not the end of
// the function
fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
