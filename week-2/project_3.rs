fn main() {
	let p:f64 = 510_000.0;
	let r:f64 = 0.05;
	let n:f64 = 3.0;

	let a = p * (1.0 - r).powf(n); 
	println!("The new value of the TV set after is: {}", a);

}

