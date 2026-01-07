use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let mut matrix = Vec::new();

    for _ in 0..5 {
        let mut row = Vec::new();
        for _ in 0..5 {
            let number: i32 = it.next().unwrap().parse().unwrap();
            row.push(number);
        }
        matrix.push(row);
    }

    for i in 0..5 {
        for j in 0..5 {
            if matrix[i][j] == 1 {
                let moves = (i as i32 - 2).abs() + (j as i32 - 2).abs();
                println!("{}", moves);
                return;
            }
        }
    }
}
