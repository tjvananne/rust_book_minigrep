
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // &args[0] is the path of the executable we're executing
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    
    println!("With text:\n{}", contents);
}


// ==== execution of package ===== 

// cargo run
// runtime error: index out of bounds error

// cargo run test poem.txt
// Searching for test
// In file: poem.txt
// With text:
// I'm nobody! Who are you?
// Are you nobody, too?
// Then there's a pair of us - don't tell!
// They'd banish us, you know.

// How dreary to be somebody!
// How public, like a frog
// To tell your name the livelong day
// To an admiring bog!

