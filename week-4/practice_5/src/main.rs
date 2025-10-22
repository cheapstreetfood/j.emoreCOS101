use std::io;

fn main() {
    let mut input = String::new();

    println!("General height consensus! How tall are you?");
    println!("\nEnter your Height (in centimeters):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:u8 = input.trim().parse().expect("Not a valid number");

    if height >= 150 && height <= 170 
    {
       println!("You are of average height person");  
    }
     else if height > 170 && height <= 195 
     {
        println!("You are tall");
     }
     else if height < 150 && height > 100
     {
        println!("You are dwarf");
     }
     else
     {
        println!("Abnormal height");
     }

}


