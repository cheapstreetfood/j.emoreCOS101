fn main() {
    let typed_tuple : (&str,f32,u8) = ("Rust",3.12,100);
    println!("Tuple contents = {:?}",typed_tuple);

    let untyped_tuple = ("Rust","ambersand",120);
    println!("{:?}",untyped_tuple);

    println!("Value at index 0 = (typed){:?}",typed_tuple.0);

    println!("Value at index 1 = {:?}",typed_tuple.1);

    println!("Value at Index 2 ={:?}",typed_tuple.2);
    
}
