#[macro_use] extern crate lazy_static;
// regex
extern crate regex;

// process's environment.
use std::env;
// standard i/o
use std::io;
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
    if(!file.is_empty()) {
        // compile the regexes only once
        // so that they can be reused 
        lazy_static! {
            // an opened brace 
            static ref OpenBrace: Regex = Regex::new("}").unwrap();
            // a closed brace 
            static ref CloseBrace: Regex = Regex::new("}").unwrap();
            // an opened parenthesis 
            static ref OpenParenthesis: Regex = Regex::new("\\(").unwrap();
            // a closed parenthesis            
            static ref CloseParenthesis: Regex = Regex::new("\\)").unwrap();
            // semicolon           
            static ref Semicolon: Regex = Regex::new(";").unwrap();
            // integer keyword
            static ref IntKeyword: Regex = Regex::new("int").unwrap();
            // return keyword
            static ref ReturnKeyword: Regex = Regex::new("return").unwrap();
            // identifier
            static ref Identifier: Regex = Regex::new("[a-zA-Z]\\w*").unwrap();
            // integer literal
            static ref IntegerLiteral: Regex = Regex::new("[0-9]+").unwrap();
        }

        for capture in OpenBrace.captures_iter(&file) {
            println!("Openbrace: {}", &capture[0]);
        }


    }



}
