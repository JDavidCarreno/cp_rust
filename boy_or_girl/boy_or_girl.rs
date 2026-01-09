use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let name = input.trim();

    let mut distinct_chars = HashSet::new();
    for c in name.chars() {
        distinct_chars.insert(c);
    }

    if distinct_chars.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}
