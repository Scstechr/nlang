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
pub const ID: &str = "ID"; // add, foobar, x, y, ...
pub const INT: &str = "INT";

// Operator
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const AST: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

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
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const RETURN: &str = "RETURN";
