fn main() {
   let mut num:u8 = 5;
   mutate_num_to_zero(&mut num);
   println!("The value of no is:{}",num);
}

fn mutate_num_to_zero(param_num:&mut u8) {
    *param_num = *param_num*0;
    println!("param_num value is :{}",param_num);
}
