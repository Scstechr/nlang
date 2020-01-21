#![allow(non_snake_case)]
#![allow(dead_code)]

pub type TokenType = String;

pub struct Token {
    Type: TokenType,
    Literal: String,
}

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";

// Identifier and Literal
const IDENT: &str = "IDENT"; // add, foobar, x, y, ...
const INT: &str = "INT";

// Operator
const ASSIGN: &str = "=";
const PLUS: &str = "+";

// Deliminator
const COMMA: &str = ",";
const SEMICOLON: &str = ";";

// Prances and Braces
const LPAREN: &str = "(";
const RPAREN: &str = ")";
const LBRACE: &str = "{";
const RBRACE: &str = "}";

// Keywords
const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";
