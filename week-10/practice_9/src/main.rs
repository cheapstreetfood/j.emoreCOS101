
struct Rectangle {
    width:u16, height:u16    
}


impl Rectangle {
    fn area(&self)->u16{
        self.width * self.height
    }
}
fn main() { 

let small = Rectangle {
    width:10,
    height:30
};

println!("width is {} \n height is {} \n area of Rectangle is {}",small.width,small.height,small.area());
}


