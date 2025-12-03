fn main() { 
let x = vec![10,31,23];
borrow_vector(&x);

println!("Printing the value from the main() x[2]={}",x[2]);
println!("\n***********************************");
}

fn borrow_vector(z:&Vec<u8>){
    println!("\n***********************************");
    println!("Inside the print_vector function {:?} \n",z);
    println!("-----------------------------------");
}