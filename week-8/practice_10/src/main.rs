
fn main() {
    let t:(u8,bool,f32) = (70,false,36.7);
    print(t);
}

fn print(x:(u8,bool,f32)) {
    println!("This is Bob");
    let (age,can_bake,baking_success_ratio) = x;
    println!("Age is {},can he bake ? {},baking success ratio is {}",age,can_bake,baking_success_ratio);

}