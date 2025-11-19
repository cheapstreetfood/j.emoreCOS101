fn main() {
    let v = vec![3,5,6,80,2,79];
    let x = vec![3,5,2,2,42,9];

    for index in 0..6 {
        let sum = v[index] + x[index];
        println!("{:?}",sum);
    }
}

