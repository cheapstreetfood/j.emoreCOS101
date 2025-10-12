fn main(){
	let toshiba = 450_000.00 * 2;
	let mac = 1_500_000.00;
	let hp = 750_000.00 * 3;
    let dell = 2_850_000.00 * 3;
    let acer = 250_000.00;

    let sum = toshiba + mac + hp + dell + acer; 
    let average = sum / 10; 

    println!("The sum of the sales is {} , and the average is {}",sum,average);
}