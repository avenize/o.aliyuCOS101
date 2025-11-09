use std::io;

fn main() {
    //printing out the menu
    println!("FOOD MENU");
    println!("========================");
    println!("Poundo yam and Edinkaiko Soup   #3,200     Item code:P");
    println!("Fried rice and chicken          #3,000     Item code:F");
    println!("Amala and Ewedu Soup            #2,500     Item code:A");
    println!("Eba and Egusi Soup              #2,000     Item code:E");
    println!("White Rice and Stew             #2,500     Item code:W");
    println!("=======================");


    //input food choice
    let mut choice = String::new();
    println!("Enter the letter for the choice{}", choice);
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim().to_uppercase();
    

    let (food, price) = match choice.as_str() {
        "P" => ("Poundo Yam/Edinkaiko Soup", 3200),
        "F" => ("Fried Rice and chicken", 3000),
        "A" => ("Amala and Ewedu Soup", 2500),
        "E" => ("Eba and Egusi Soup", 2000),
        "W" => ("White Rice and Stew", 2500),
        _ => {
            println!("Invalid choice. Please enter a valid option");
            return;
        }
    };


    //input quantity
    let mut qty = String::new();
    println!("Enter the Quantity{}", qty);
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty :u32 = qty.trim().parse().expect("Please enter a valid number");
    

    let total :f32 = price as f32 * qty as f32;

    //apply discount
    if qty <= 0 {
        println!("Invalid: You entered a number less than one");
    } else if total <= 10000.0 {
        println!("You ordered: {}", food);
        println!("Quantity ordered: {}", qty as f32);   
        println!("The total cost is: {}", total);
    } else if total > 10000.0 {
        let discountedtotal = total - total as f32 * 0.05;
        println!("You ordered: {}", food);
        println!("Quantity ordered: {}", qty as f32);   
        println!("The total cost is: {}", discountedtotal);
    } else {
        println!("Error");
    }
}
