fn main() {
    let v = vec![2, 3, 4, 5, 6, 7];

    let n_vec: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).map(|x| x * 2).collect();

    println!("{:?}", n_vec);

    let cuadrados_mayores_tres: Vec<i32> = v.iter().filter(|x| **x > 3).map(|x| *x * *x).collect();

    println!("{:?}", cuadrados_mayores_tres);
}
