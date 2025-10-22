
fn main() {
    println!("X counter(1-15)");

    let mut x = 0;
    loop {
        x+=1;
        println!("x={}",x);

        if x==15 {
            break;
        }
    }
}
