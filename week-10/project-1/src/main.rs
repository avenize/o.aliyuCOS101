struct Laptops {
    name:String,
    price:u32
}


impl Laptops {
    fn sum(&self)->u32 {
        self.price * 3 
    }
}
fn main() {
    let laptop1 = Laptops {
        name:String::from("HP"),
        price:650_000
    };

    let laptop2 = Laptops {
        name:String::from("IBM"),
        price:755_000
    };

    let laptop3 = Laptops {
        name:String::from("Toshiba"),
        price:550_000
    };

    let laptop4 = Laptops {
        name:String::from("Dell"),
        price:850_000
    };

    println!("The sum of 3 {} laptops is {}, the sum of 3 {} laptops is {}, the sum of 3 {} laptops is {}, the sum of 3 {} laptops is {}", laptop1.name, laptop1.sum(), laptop2.name, laptop2.sum(), laptop3.name, laptop3.sum(), laptop4.name, laptop4.sum());

    println!("The total cost if a customer purchases 3 laptops each from a brand is {}", laptop1.sum() + laptop2.sum() + laptop3.sum() + laptop4.sum());



}
