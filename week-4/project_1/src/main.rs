use std::io;  //import io module


fn main() {
    //create mutable variables
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();


    println!("Enter value for a:");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read input");
    let a: f64 = a
    .trim()
    .parse()
    .expect("Not a valid number");

    println!("Enter value for b:");
    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read input");
    let b: f64 = b
    .trim()
    .parse()
    .expect("Not a valid number");

    println!("Enter value for c:");
    io::stdin()
    .read_line(&mut c)
    .expect("Failed to read input");
    let c: f64 = c
    .trim()
    .parse()
    .expect("Not a valid number");


    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let _root_1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let _root_2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", _root_1, _root_1);
    }else if discriminant == 0.0 {
        let _root = -b / (2.0 * a);
        println!("Discriminant {}", discriminant);
        println!("One real root: {:.2}", _root);
    } else {
        println!("No real roots");
    }
}




