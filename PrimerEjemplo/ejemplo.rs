use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let first : i32 = it.next().unwrap().parse().unwrap();
    let mut last = first;

    for _ in 1..n {
        last = it.next().unwrap().parse().unwrap();
    }
    print!("{} {} {}",first, last, first + last);
}

