use std::io;

fn main() {

    println!("Conversion de Celcius en Fahrenheit :");
    println!("1. From celcius to Fahrenheit");
    println!("2. From Fahrenheit to Celcius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Entrez une valeur svp");

    let choice = choice.trim();

    if choice == "1" {

        println!("Entrez la valeur en celcius:");
        let celcius_val = take_user_input();
        let celcius_val: f32 = celcius_val.trim().parse().expect("Vous n'avez pas entré un nombre");
        println!("la valeur en fahrenheit est : {}", from_celcius_to_farenheit(celcius_val));

    } else if choice == "2" {

        println!("Entrez la valeur en fahrenheit:");
        let fahrenheit_val = take_user_input();
        let fahrenheit_val: f32 = fahrenheit_val.trim().parse().expect("Impossible de convertir");
        println!("la valeur en fahrenheit est : {}", from_fahrenheit_to_celcius(fahrenheit_val));

    }


}

fn take_user_input() -> String {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Valeur incorrecte");

    number
}

fn from_celcius_to_farenheit(cel: f32) -> f32 {
    (cel * 9.0/5.0) + 32.0
}

fn from_fahrenheit_to_celcius(fah: f32) -> f32 {
    (fah - 32.0) * 5.0/9.0
}
