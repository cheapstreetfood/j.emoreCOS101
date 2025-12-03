fn add_one(e: &mut u8) {
    *e+= 1;   
}

fn main() {
    let mut i:u8 = 3;
    add_one(&mut i);
    println!("{}",i);
}