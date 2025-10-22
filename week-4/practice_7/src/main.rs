use std::io;

fn main() {
    println!("Number guesser and Counter");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut num:u8 = input1.trim().parse().expect("Not a valid number");

    while num < 10 {
        println!("Inside the loop number value is {}",num);
        num+=1;
    }
    println!("Outside loop number value is {}",num);
}
