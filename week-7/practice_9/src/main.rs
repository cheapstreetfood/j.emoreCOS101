fn main() {
    let arr:[u8;4] = [10,20,30,40];
    println!("Array is{:?}",arr);
    println!("Array size is :{}",arr.len());

    for val in arr.iter(){
        println!("The value is:{}",val);
    }
}
