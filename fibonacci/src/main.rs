use std::io;


fn main() {

    println!("Calculate fibonacci of:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Incorrect value, enter a number please");

    let number: u128 = number.trim().parse().expect("Cannot this value into a number");

    println!("fibonacci({}) = {}", number, fibonacci(number));
}


fn fibonacci(number: u128) -> u128{ // O(n) complexity
    let mut memo = [0, 1];

    if number < 2 {
        return number;
    }

    let mut somme :u128;
    for _ in 1..number {
        somme = memo[0] + memo[1];
        memo[0] = memo[1];
        memo[1] = somme;
    }
    memo[1]
}


fn fibonacci_rec(number: u32) -> u32 { // exponential time
    if number < 2 {
        return number;
    }
    fibonacci_rec(number - 2) + fibonacci_rec(number - 1)
}
