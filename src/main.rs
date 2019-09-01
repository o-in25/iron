#[macro_use] extern crate lazy_static;
// regex
extern crate regex;

// process's environment.
// use std::env;
// standard i/o
// use std::io;

// use the file system  
use std::fs;
// regular expressions

use regex::Regex;
// vectors
use std::vec::Vec;
mod lexer;

// function that accepts a file and returns 
// a list of tokens
fn main() {
    println!("Beginning...");
    read_from_file("./etc/return_2.c");
}

// will lex a file given its
// path as a string
fn read_from_file(path: &str) {
    let file = fs::read_to_string(path).expect("Error parsing file.");
    if !file.is_empty() {
        let vec: Vec<&str> = file.split_whitespace().collect();
        // now that we have split the prospective
        // tokens by the white space, time to
        // parse the strings
        lexer::test();

    }
}
