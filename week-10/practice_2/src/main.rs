fn main() {
    let v = vec![20,50,80];

    let v2 = v;

    display(v2.clone());

    println!("In main {:?}",v2);
}

fn display(v:Vec<i32>){
    println!("Inside the display {:?}",v);
}
