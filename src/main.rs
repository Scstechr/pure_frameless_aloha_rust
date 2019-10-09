extern crate pure_fa;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("cargo run --release <output>");
    } else {
        let output = &args[1];
        println!("#Output: {}", output);
        pure_fa::run(&output);
    }
}