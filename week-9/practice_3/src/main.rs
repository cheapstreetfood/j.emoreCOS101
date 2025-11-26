use std::fs;

fn main() {
    fs::remove_file("../code_folders/removed_file.txt").expect("could not remove file");
    println!("file is removed");
}

