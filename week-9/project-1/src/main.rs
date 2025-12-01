use std::fs::File;
use std::io::Write;


fn main() {
    //drink categories
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["legend", "Turbo king", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "MAlta Gold", "Fayrouz"];

    //Create file
    let mut file = File::create("data.txt").expect("unable to create file");

    file.write_all(b"Lager:\n").unwrap();
    for drink in lager {
        write!(file, "{}\n", drink).unwrap();
    }
    file.write_all(b"Stout:\n").unwrap();
    for drink in stout {
        write!(file, "\n", drink).unwrap();
    }
    file.write_all(b"Non-alcoholic:\n").unwrap();
    for drink in non_alcoholic {
        write!(file, "{}\n", drink).unwrap();
    }
    println!("data.txt created");
}
