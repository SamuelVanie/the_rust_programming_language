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
        let mut celcius_val = String::new();

        io::stdin()
            .read_line(&mut celcius_val)
            .expect("Valeur incorrecte");

        let celcius_val: f32 = celcius_val.trim().parse().expect("Impossible de convertir");

        let farenheit_value = (celcius_val * 9.0/5.0) + 32.0;
        println!("la valeur en fahrenheit est : {farenheit_value}");

    } else if choice == "2" {

        println!("Entrez la valeur en fahrenheit:");
        let mut fahrenheit_val = String::new();

        io::stdin()
            .read_line(&mut fahrenheit_val)
            .expect("Valeur incorrecte");

        let fahrenheit_val: f32 = fahrenheit_val.trim().parse().expect("Impossible de convertir");

        let celcius_val = (fahrenheit_val - 32.0) * 5.0/9.0;
        println!("la valeur en fahrenheit est : {celcius_val}");

    }


}
