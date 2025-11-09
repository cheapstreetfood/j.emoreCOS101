use std::io;
fn main() {

    println!("         Menu                     Price");
    println!("P = Poundo Yam/Edinkaiko Soup    -₦3,200");
    println!("F = Fried Rice & Chicken         -₦3,000");
    println!("A = Amala & Ewedu Soup           -₦2,500");
    println!("E = Eba & Egusi Soup             -₦2,000");
    println!("W = White Rice & Stew            -₦2,500");


    let mut food_code = String::new();
    println!("Please enter item code (P/F/A/E/W): ");
    io::stdin().read_line(&mut food_code).expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase();


    let mut quantity = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:u32 = quantity.trim().parse().expect("Please enter a valid number");


    let mut item_name = "";
    let mut unit_price: u32 = 0;

    if food_code == "P" {
        let item_name = "Poundo Yam/Edinkaiko Soup";
        let unit_price:u32 = 3200;
    }
    if food_code == "F" {
        let item_name = "Fried Rice & Chicken";
        let unit_price:u32 = 3000;
    }
    if food_code == "A" {
        let item_name = "Amala & Ewedu Soup";
        let unit_price:u32 = 2500;
    }
    if food_code == "E" {
        let item_name = "Eba & Egusi Soup";
        let unit_price:u32 = 2000;
    }
    if food_code == "W" {
        let item_name = "White Rice & Stew";
        let unit_price:u32 = 2500;
    }
    let mut total_charge:u32 = unit_price * quantity ;

    let mut discount:f32 = 0.0;

    let mut final_total:f32 = total_charge as f32;

    if total_charge > 10000 {
        let discount = total_charge as f32 * 0.05;
        let final_total:f32 = total_charge as f32 - discount;
    }

    println!("\n ORDER ");
    println!("Item: {}", item_name);
    println!("Quantity: {}", quantity);
    println!("Unit Price: N{}", unit_price);
    println!("Total: N{}", total_charge);
    
    if discount > 0.0 {
        println!("Discount (5%): N{}", discount);
        println!("Final Total: N{}", final_total);
    } else {
        println!("Final Total: N{}", final_total);
    
    }



}

