use std::io;
use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    println!("Please write your name and a message to the file: ");
    let mut add_stuff = String::new();
    io::stdin().read_line(&mut add_stuff).expect("Input failed");
    let added:&str = &add_stuff;
    
    let mut file = OpenOptions::new().append(true).open("../code_folders/appending_message.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.\n".as_bytes()).expect("write failed");
    file.write_all("\nHere is what the user added to the document:\n".as_bytes()).expect("Write failed");
    file.write_all(added.as_bytes());
    println!("file appendage success!");
}
