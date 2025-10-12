fn main (){
	let principal = 510_000;
	let rate = 5; 
	let n = 3; 
	let a = principal * ((1 - (rate/100)) ^ n);
	let depreciation = principal - a ;
	println!("The depreciation is N{}",depreciation);
}