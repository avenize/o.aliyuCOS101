fn main() {
	let qty = [2,1,3,3,1];
	let amt = [450_000, 1_500_000, 750_000, 2_850_000, 250_000];
	let t_amt = [amt[0]*qty[0], amt[1]*qty[1], amt[2]*qty[2], amt[3]*qty[3], amt[4]*qty[4]];

	let _sum_amt :i32 = t_amt.iter().sum(); // sum 
	let _sum_qty :i32 = qty.iter().sum();   // sum of qty
	let average :i32 = _sum_amt/_sum_qty;

	println!("{}", _sum_amt);
	println!("{}", average);
}
