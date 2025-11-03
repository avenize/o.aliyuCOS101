use std:io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nAre you experienced or inexperienced (Y/N):");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experienced = input.trim();

    println!("\nEnter your age:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:f64 = input2.trim().parse()

}
