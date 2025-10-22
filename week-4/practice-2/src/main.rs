use std::io;

fn main() {
   let  mut input1 = String::new();
   let  mut input2 = String::new();
   let  mut input3 = String::new();

   println!("\n Triangle Area Calculator");
   println!("\nPlease enter first edge of triangle: ");
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let a:f32 = input1.trim().parse().expect("Not a valid number");

   println!("Please enter second edge of triangle: ");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let b:f32 = input2.trim().parse().expect("Not a valid number");


   println!("Please nter third edge of triangle: ");
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let c:f32 = input3.trim().parse().expect("Not a valid number");

   let s:f32 = (a + b + c) / 2.0;
   let mut area:f32 = s * (s - a) * (s - b) * (s - c);
   area = area.sqrt();

   println!("Area of the triangle: {}",area);







}
