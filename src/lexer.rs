
// regex
extern crate regex;

// regular expressions
use regex::Regex;
// vectors
use std::vec::Vec;

pub enum NameT {
    OpenBrace,
    CloseBrace,
    OpenParenthesis,
    CloseParenthesis,
    Semicolon,
    IntegerKeyword,
    ReturnKeyword,
    Identifier,
    IntegerLiteral
}

pub struct TokenT<'a> {
    name: NameT,
    value: &'a str
}

// gets the next token
// in the sequence
pub fn next(sequence: &str) -> TokenT {
    lazy_static! {
            // an opened brace
            static ref OPEN_BRACE: Regex = Regex::new("{").unwrap();
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

    match sequence {
        sequence if OPEN_BRACE.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: "{"
            };
            return t;
        }
        sequence if CLOSE_BRACE.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: "}"
            };
            return t;
        }
        sequence if OPEN_PARENTHESIS.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: "("
            };
            return t;
        }
        sequence if CLOSE_PARENTHESIS.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: ")"
            };
            return t;
        }
        sequence if SEMICOLON.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: ";"
            };
            return t;
        }
        sequence if INTEGER_KEYWORD.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: "int"
            };
            return t;
        }
        sequence if RETURN_KEYWORD.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: "return"
            };
            return t;
        }
        sequence if IDENTIFIER.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: sequence
            };
            return t;
        }
        sequence if INTEGER_LITERAL.is_match(sequence) => {
            let t = TokenT {
                name: NameT::OpenBrace,
                value: sequence
            };
            return t;
        }
        _ => {}
    }

    let t = TokenT {
        name: NameT::OpenBrace,
        value: ""
    };
    return t;

}
