
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}


// ==== execution of package ===== 

// cargo run
// ["target\\debug\\rust_book_minigrep.exe"]

// cargo run my sweet args
// ["target\\debug\\rust_book_minigrep.exe", "my", "sweet", "args"]

