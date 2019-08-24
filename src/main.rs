use std::io;
use std::env;
// use the file system
use std::fs;

// function that accepts a file and returns a list of tokens
fn main() {
    println!("Begining...");
    readFromFile("./etc/return_2.c");
}

fn readFromFile(path: &str) {
    let contents = fs::read_to_string(path).expect("Whoops");
    println!("With text:\n{}", contents);

}
