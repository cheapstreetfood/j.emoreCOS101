use std::io;

fn main() {
    println!("Quadratic Equation Calculator");
    println!("\n Please input a:");
    let mut input1 = String::new(); 
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("\n Please input b:");
    let mut input2 = String::new(); 
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("\n Please input c:");
    let mut input3 = String::new(); 
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let mut d:f32 = (b*b) - (4.0 * a * c);// discriminant
    if d == 0.0 {
        let sol:f32 = -b/(2.0 * a);
        println!("There is exactly one real root:{}",sol);
    } else if d > 0.0 {
        println!("There are two distinct roots.");
        let sol1:f32 = (-b + d)/2.0;
        let sol2:f32 = (-b - d)/2.0; 
        println!("The roots are {} and {}",sol1,sol2);
    }else if d < 0.0 {
        println!("There are no real roots");
        let sol_real:f32 = -b/(2.0 * a);
        d = d.abs(); // to make the negative determinant positive
        let sol_im:f32 = d.sqrt() / (2.0 * a);
        println!("First root {} +i {}",sol_real,sol_im);
        println!("\nSecond root {} -i {}",sol_real,sol_im);
    }else if a == 0.0  ||b == 0.0 || c == 0.0 {
        print!("Error");
    }else {
        println!("Error");
    }
}
