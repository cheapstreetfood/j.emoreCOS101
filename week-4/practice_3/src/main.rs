
use std::io; 

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nTriangle Area Calculator");

    println!("Please enter base: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Please enter height: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let height:f32 = input2.trim().parse().expect("Not a valid number");

    if base > 0.0 {
        let area:f32 = (base * height) / 2.0 ; 
        println!("Area of the triangle : {}", area); 
    }else {
        println!("Something went wrong D:");
        
    }


}
