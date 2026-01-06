fn main() {
    let mi_vector = vec![1, 2, 3, 4];

    let nuevo_vector = recibir(&mi_vector);

    for x in &nuevo_vector {
        println!("{}", &x);
    }

    println!("{:?}", mi_vector);
}

fn recibir(v: &Vec<i32>) -> Vec<i32> {
    let mut otro_vector = Vec::new();
    for x in v {
        otro_vector.push(*x);
    }
    otro_vector
}

fn suma_uno(v: &mut Vec<i32>) {
    for x in v {
        *x += 1;
        println!("{}", x);
    }
}
