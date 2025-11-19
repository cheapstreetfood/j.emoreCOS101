fn main() {
    let mut alphabet_names = ["Amanda","Abigail","Bernice","Bennet","Casey","Carl","Dominga","Dominique"];
    println!("Alphabet Squad and their divisions!");
    let a_names = &mut alphabet_names[0..2];
    println!("A division ! {:?}",a_names);

    a_names[1] = "Ahmed";

    println!("A member was replaced! New A division is {:?} !",a_names);
}


