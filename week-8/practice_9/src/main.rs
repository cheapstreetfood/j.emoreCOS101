fn main() {
    let y:(u8,bool,f32) = (50,true,41.1);
    print(y);
}

fn print(x:(u8,bool,f32)) {
    println!("Inside print method");
    println!("{:?}",x);
}
