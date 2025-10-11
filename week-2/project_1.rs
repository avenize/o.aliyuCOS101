fn main() {
	let p:f64 = 520_000_000.0;
	let r:f64 = 0.01;
	let n:f64 = 5.0;

	let a = p * (1.0 + r).powf(n); 
	let ci = a - p;
	println!("The compound interest for {} years at 10% per annum on a N {} loan is: {}", n, p, ci);

}