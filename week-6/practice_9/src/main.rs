fn main() {
    let a:u8 = 10;
    let b:u8 = 20;

    println!("Value of a:{}",a);
    println!("Value of b:{}",b);

    let mut res = a>b ; 
    println!("a greater than b :{}",res);

    
    res = a<b ;
    println!("a lesser than b :{}",res);

    res = a>=b ; 
    println!("a greater than or equal to than b :{}",res);

    res = a<=b ; 
    println!("a lesser than or equal to than b :{}",res);

    res = a==b ; 
    println!("a is equal to than b :{}",res);

    res = a!=b ; 
    println!("a is not equal to than b :{}",res);
}


