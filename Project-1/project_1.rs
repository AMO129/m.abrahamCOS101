fn main(){
	let p:f64 = 520000.0;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// compound interest
	let am = p * (1.0 + (r/100.0)) * t;
	println!("Ammount is {}", am);
	let cl = am - p;
	println!("Compound interest is {}", cl);

}