use std::env;

use rust_programs::{run_001, run_002, run_003, run_004, run_005, run_006, run_007};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "001" => {
            run_001::run();
        }
        "002" => {
            run_002::run();
        }
        "003" => {
            run_003::run();
        }
        "004" => {
            run_004::run();
        }
        "005" => {
            run_005::run();
        }
        "006" => {
            run_006::run();
        }
        "007" => {
            run_007::run();
        }
        "008" => {
            run_007::run();
        }
        _ => {
            println!("Invalid argument");
        }
    }
}
