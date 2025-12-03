
struct Employee {
    company:String,
    ceo:String,
    age:u8
}

fn main() {
    let emp1 = Employee {
        company:String::from("Nintendo Ltd"),
        ceo:String::from("Shuntaro Furukawa"),
        age:53
    };
    let emp2 = Employee {
        company:String::from("Hoyoverse"),
        ceo:String::from("Da Wei"),
        age:38
    };

    display(emp1);
    display(emp2);   
}

fn display(emp :Employee){
    println!("Name is :{} company is {} age is {}",emp.ceo,emp.company,emp.age);
}
