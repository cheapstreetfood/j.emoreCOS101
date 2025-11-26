fn main(){
	let principal = 520000000 ; 
	let n = 5;
	let rate = 10;
	let a = principal * ((1 + (rate/100)) ^ n); 
	let compound_i  = a - principal;
	println!("Compound interest is N{}",compound_i);

}
