
// regex
extern crate regex;

// regular expressions
use regex::Regex;
// vectors
use std::vec::Vec;

pub enum NameT {
    OPEN_BRACE,
    CLOSE_BRACE,
    OPEN_PARENTHESIS,
    CLOSE_PARENTHESIS,
    SEMICOLON,
    INTEGER_KEYWORD,
    RETURN_KEYWORD,
    IDENTIFIER,
    INTEGER_LITERAL
}

pub struct TokenT<'a> {
    name: NameT,
    value: &'a str
}

pub fn parse(sequence: &str) -> TokenT {
    let t = TokenT {
        name: NameT::RBRACE,
        value: "10"
    };
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

    return t;

}
