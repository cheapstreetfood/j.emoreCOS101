struct Consignment {
    name:String,
    cost:u32
}

fn main() {
   let hp = Consignment{
         name:String::from("hp"),
         cost:650_000
   };

   let ibm = Consignment{
         name:String::from("IBM"),
         cost:755_000  
   };

   let toshiba = Consignment{
         name:String::from("Toshiba"),
         cost:550_000
   };

   let dell = Consignment{
         name:String::from(" Dell"),
         cost:850_000
   };

   let quantity:u32 = 3;
    
    // Calculate cost for each brand: price × 3
    let hp_cost = hp.cost * quantity;
    let ibm_cost = ibm.cost * quantity;
    let toshiba_cost = toshiba.cost * quantity;
    let dell_cost = dell.cost * quantity;
    
    // Calculate total cost by adding all brand costs
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;
    
    // Display the results
    println!("Purchase Summary (3 laptops from each brand):");
    println!("{}: ₦{} × 3 = ₦{}", hp.name, hp.cost, hp_cost);
    println!("{}: ₦{} × 3 = ₦{}", ibm.name, ibm.cost, ibm_cost);
    println!("{}: ₦{} × 3 = ₦{}", toshiba.name, toshiba.cost, toshiba_cost);
    println!("{}: ₦{} × 3 = ₦{}", dell.name, dell.cost, dell_cost);
    println!("\nTotal Cost: ₦{}", total_cost);
}



