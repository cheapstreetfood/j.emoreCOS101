fn main() {
   let v = vec!['B','A','C','H','E','L','O','R'];

   let mut input1 = String::new();

   println!("Enter an index value between (0-7)");
   std::io::stdin().read_line(&mut input1).expect("Not a valid string");

   let index:usize = input1.trim().parse().expect("Invalid Input");

   let ch: char = v[index];

   print!("{} is the character for index [{}]\n",ch,index );
}
