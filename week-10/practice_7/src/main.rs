struct Employee {
    name:String,
    company:String,
    age:u8
}

fn main() {
    let emp1 = Employee {
        company:String::from("Macs & Spencers"),
        name:String::from("Alabeko Ekoko"),
        age:31
    };
    println!("Name = {} \n",emp1.name);
    println!("Company = {}",emp1.company);
    println!("Age = {} ",emp1.age);   
}