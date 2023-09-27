use std::collections::HashMap;

fn main() {
    let mut numbers = vec![3, 4, 1, 9, 2, 5, 7, 8, 6, 1];

    let mut map_number: HashMap<i32, i32> = HashMap::new();

    for number in numbers.iter() {
	let nb_of_occurences = map_number.entry(*number).or_insert(0);
	*nb_of_occurences += 1;
    }
    numbers.sort();
    let median = numbers.get(numbers.len() / 2).unwrap();
    println!("The median is {median} and the one with the most appearances is {}", map_number.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap());

}
