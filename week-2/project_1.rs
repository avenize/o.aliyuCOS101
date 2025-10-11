fn main() {
	let p = 520_000_000;
	let r = 10%;
	let n = 5;

	let a = p(1 + (r/100))^^n; //n.powi(5)
	let ci = a - p;

}