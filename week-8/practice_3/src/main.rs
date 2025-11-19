fn main() {
    let v = vec!['E','X','P','E','R','T','I','S','A'];

    let mut input = String::new();
    println!("Enter an index value between (0-8)");
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let index:usize = input.trim().parse().expect("Invalid input");

    let ch: Option<&char> = v.get(index);
    value(ch);

}

fn value(n:Option<&char>) {
    println!("Element of vector is {:?}",n);
}