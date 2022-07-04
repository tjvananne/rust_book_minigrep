
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // &args[0] is the path of the executable we're executing
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file: {}", filename);
}


// ==== execution of package ===== 

// cargo run
// runtime error: index out of bounds error

// cargo run mystring samplefile.txt
// Searching for mystring
// In file: samplefile.txt

