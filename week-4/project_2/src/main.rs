use std::io;

fn main() {
    println!("\n Annual Incentive Prompt");
    println!("\n Please enter your full name :");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Try again");

    println!("Please enter you age :");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:u8 = age.trim().parse().expect("Not a valid number");


    println!("Please enter your years of experience:");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Not a valid string");
    let experience:u8 = experience.trim().parse().expect("Not a valid number");
    let  is_experienced :bool;
    if experience >= 5 {
        is_experienced = true;
    } else {
        is_experienced = false;
    }
    let _incentive:u32 = 0;
    if is_experienced == true && age >= 40 {
        let incentive:u32 = 1_560_000;
        println!("Contratulobia, {} you Incentive is {} naira",name,incentive);
    } else if (is_experienced == true) &&  (30 > age && age > 40) {
        let incentive:u32 = 1_480_000;
        println!("Contratulobia, {} your incentive is {} naira",name,incentive);
    } else if is_experienced == true && age < 28 {
        let incentive:u32 = 1_300_000;
        println!("Contratulobia,{} your incentive is {} naira",name,incentive);
    } else if is_experienced == false {
        let incentive:u32 = 100_000;
        println!("Contratulobia,{} your incentive is {} naira",name,incentive);
    } else {
        println("Something went wrong try again");
    }

}
