
const PHRASES: [&str; 11] = [
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming"
];

const NUMBERS: [&str; 11] = [
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "11th",
    "12th"
];


fn build_intermediate(cur_number: usize){
   for i in (0..cur_number + 1).rev() {
       println!("{}", PHRASES[i]);
   }
}

fn repeated_part(count: usize) {
    println!("On the {} day of Christmas", NUMBERS[count]);
    println!("My true love sent to me");
    build_intermediate(count);
    println!("And a partridge in a pear tree");
    println!("\n");
}


fn main() {
    println!("On the first day of Christmas");
    println!("My true love sent to me");
    println!("A partride in a pear tree");
    println!("\n");

    let verses_number = 11;
    for i in 0..verses_number {
        repeated_part(i);
    }
}
