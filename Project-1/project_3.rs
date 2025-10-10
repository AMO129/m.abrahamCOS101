fn main(){
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// value of TV after three years
	let am = p * (1.0 - (r/100.0)).powf(t);
	println!("Ammount is {:.2}", am);
	

}