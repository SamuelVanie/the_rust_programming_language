use std::io;

fn main() {
    println!("Entrez la valeur en celcius:");
    let mut celcius_val = String::new();


    io::stdin()
        .read_line(&mut celcius_val)
        .expect("Valeur incorrecte");

    let celcius_val: u32 = celcius_val.trim().parse().expect("Impossible de convertir");

    let farenheit_value = (celcius_val * 9/5) + 32;
    println!("la valeur en fahrenheit est : {farenheit_value}");

}
