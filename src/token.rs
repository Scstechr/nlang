#![allow(non_snake_case)]
#![allow(dead_code)]

pub type TokenType = String;

#[derive(Debug)]
pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifier and Literal
pub const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
pub const INT: &str = "INT";

// Operator
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

// Deliminator
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

// Prances and Braces
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
