fn main() {
   let a:u8 = 2;
   let b:u8 = 3;

   let mut result:u8;

   result = a & b;
   println!("(a & b) => {}",result);

   result = a | b;
   println!("(a | b) => {}",result);

   result = a ^ b;
   println!("(a ^ b) => {}",result);

   result = !b;
   println!("(!b) => {}",result);

   result = a << b;
   println!("(a << b) => {}",result);

   result = a >> b;
   println!("(a >> b) => {}",result);
}
