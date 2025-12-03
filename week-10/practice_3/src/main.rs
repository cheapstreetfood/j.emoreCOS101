fn main(){
    let v = vec![4,64,32,08];

    let v2 = v.clone();
    let v2_return = display(v2);
    println!("In main {:?}",v);
}

fn display(v:Vec<i32>)->Vec<i32> {
    // returning same vector
    println!("inside display{:?}",v);
    return v;   
}