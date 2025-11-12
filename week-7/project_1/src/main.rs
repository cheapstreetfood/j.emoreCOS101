use std::f32::consts::PI;
use std::io;
fn main() {
    println!("||__Shape Area Calculator__||"); 
    loop
    {
    println!("Please enter the letter code of the shape would you like to calculate:");
    println!("T = Trapezium");
    println!("R = Rhombus");
    println!("P = Parallelogram");
    println!("C = Cube");
    println!("Y = Cylinder");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let shape_code:char = input.trim().to_uppercase().parse().expect("Invalid input");

    if shape_code == 'T' {
        println!("Trapezium selected!"); 

        println!("Please enter the first base:");    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let firstbase:f32 = input2.trim().parse().expect("Invalid input");


        println!("Please enter the second base:");    
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let secondbase:f32 = input3.trim().parse().expect("Invalid input");


        println!("Please enter the height:");    
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let height:f32 = input4.trim().parse().expect("Invalid input");


        let area_t = trapezium(firstbase,secondbase,height);
        println!("The area of your trapezium with the two bases {} and {} with height {} = {}",firstbase,secondbase,height,area_t);


    }else if shape_code == 'R' { 
        println!("Rhombus selected!"); 
        println!("Please enter the first diagonal:");    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let first_diagonal: f32 = input2.trim().parse().expect("Invalid input");

        println!("Please enter the second diagonal:");    
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let second_diagonal: f32 = input3.trim().parse().expect("Invalid input");

        let area_r = rhombus(first_diagonal, second_diagonal);
        println!("The area of your rhombus with diagonals {} and {} = {}", first_diagonal, second_diagonal, area_r);

    }else if shape_code == 'P' {
        println!("Parallelogram selected!"); 
        println!("Please enter the base:");    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let base: f32 = input2.trim().parse().expect("Invalid input");

        println!("Please enter the altitude:");    
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let altitude: f32 = input3.trim().parse().expect("Invalid input");

        let area_p = parallelogram(base, altitude);
        println!("The area of your parallelogram with base {} and altitude {} = {}", base, altitude, area_p);

    }else if shape_code == 'C' {
        println!("Cube selected!"); 
        println!("Please enter the length of side:");    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let length: f32 = input2.trim().parse().expect("Invalid input");

        let vol_c = cube(length);
        println!("The volume of your cube with side length {} = {}", length, vol_c);

    }else if shape_code == 'Y' {
        println!("Cylinder selected!");
        println!("Please enter the radius:");    
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let radius: f32 = input2.trim().parse().expect("Invalid input");

        println!("Please enter the height:");  
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let height: f32 = input3.trim().parse().expect("Invalid input");

        let vol_cy = cylinder(radius, height);
        println!("The volume of your cylinder with radius {} and height {} = {}", radius, height, vol_cy);


    }else {
        println!("Please try again. Make sure to use the letter code");
    }
}
}

fn trapezium(base1:f32,base2:f32,height:f32)->f32 {
    let area_t:f32 = (height/2.0) * (base1 + base2);
    return area_t
}

fn rhombus(diagonal1:f32,diagonal2:f32)->f32 {
    let area_r:f32 = 0.5 * diagonal1 * diagonal2;
    return area_r
}

fn parallelogram(base:f32,altitude:f32)->f32 {
    let area_p:f32 = base * altitude;
    return area_p
}

fn cube(lengthofside:f32)->f32 {
    let vol_c:f32 = 6.0 * (lengthofside*lengthofside);
    return vol_c
}

fn cylinder(radius:f32,height:f32)->f32 {
    let vol_cy = PI * (radius*radius) * height; 
    return vol_cy
}