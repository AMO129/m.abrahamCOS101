fn main(){
	let ts:f64 = (450000.00) * 2.00;
	let ma:f64 = (1500000.00) * 1.00;
	let hp:f64 = (750000.00) * 3.00;
	let dl:f64 = (2850000.00) * 3.00;
	let ac:f64 = (250000.00) * 1.00;
	let tl:f64 = 10.00;

	// sum
	let sum = ts + ma + hp + dl + ac;
	println!("Sum of laptops costs is {}", sum);
	let avg = sum/tl;
	println!("Average is {}", avg);

}