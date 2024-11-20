use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "001" => { println!("001"); }
        "002" => { println!("002"); }
        "003" => { println!("003"); }
        "004" => { println!("004"); }
        "005" => { println!("005"); }
        "006" => { println!("006"); }
        "007" => { println!("007"); }
        "008" => { println!("008"); }
        _ => { println!("Invalid argument"); }
    }
}
