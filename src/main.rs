#[macro_use] extern crate lazy_static;
// regex
extern crate regex;

// process's environment.
// use std::env;
// standard i/o
//use std::io;
// use the file system
use std::fs;
// regular expressions
use regex::Regex;


// function that accepts a file and returns 
// a list of tokens
fn main() {
    println!("Begining...");
    lex("./etc/return_2.c");
}

// will lex a file given its
// path as a string
fn lex(path: &str) {
    let file = fs::read_to_string(path).expect("Error parsing file.");
    if !file.is_empty() {
        // compile the regexes only once
        // so that they can be reused 
        lazy_static! {
            // an opened brace 
            static ref OPEN_BRACE: Regex = Regex::new("}").unwrap();
            // a closed brace 
            static ref CLOSE_BRACE: Regex = Regex::new("}").unwrap();
            // an opened parenthesis 
            static ref OPEN_PARENTHESIS: Regex = Regex::new("\\(").unwrap();
            // a closed parenthesis            
            static ref CLOSE_PARENTHESIS: Regex = Regex::new("\\)").unwrap();
            // semicolon           
            static ref SEMICOLON: Regex = Regex::new(";").unwrap();
            // integer keyword
            static ref INTEGER_KEYWORD: Regex = Regex::new("int").unwrap();
            // return keyword
            static ref RETURN_KEYWORD: Regex = Regex::new("return").unwrap();
            // identifier
            static ref IDENTIFIER: Regex = Regex::new("[a-zA-Z]\\w*").unwrap();
            // integer literal
            static ref INTEGER_LITERAL: Regex = Regex::new("[0-9]+").unwrap();
        }

        for capture in OPEN_BRACE.captures_iter(&file) {
            println!("Openbrace: {}", &capture[0]);
        }


    }



}
