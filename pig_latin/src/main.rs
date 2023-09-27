fn to_pig_latin(s: &str) {
    let vowels = vec!['a', 'e', 'i', 'o', 'y'];
    if !vowels.contains(&s.chars().nth(0).unwrap()) {
	println!("{}-{}ay", s.chars().skip(1).collect::<String>(), s.chars().nth(0).unwrap()); 
    } else {
	println!("{}-hay", s);
    }
}

fn main() {
    to_pig_latin(&String::from("first"));
}
