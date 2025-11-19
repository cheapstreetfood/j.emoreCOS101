fn main() {
    let alphabet_names = ["Amanda","Abigail","Bernice","Bennet","Casey","Carl","Dominga","Dominique"];
    println!("Alphabet Squad and their divisions!");
    let a_names = &alphabet_names[0..2];
    println!("A division ! {:?}",a_names);

    let b_names = &alphabet_names[2..4];
    println!("B division!{:?}",b_names);

    let c_names = &alphabet_names[4..6];
    println!("C division!{:?}",c_names);

    let d_names = &alphabet_names[6..];
    println!("D division!{:?}",d_names);



}
