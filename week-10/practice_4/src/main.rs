fn main() {
    let v = vec![24,23,64,87,34];
    print_vector(v.clone());
    println!("{}",v[0]);
}

fn print_vector(x:Vec<i32>) {
    println!("Inside print_vector function {:?}",x);
}
