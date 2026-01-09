use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let first_string = it.next().unwrap().to_lowercase();
    let second_string = it.next().unwrap().to_lowercase();

    if first_string < second_string {
        println!("{:?}", -1);
    } else if first_string > second_string {
        println!("{:?}", 1);
    } else {
        println!("{:?}", 0);
    }
}
