#![allow(non_snake_case)]
#![allow(dead_code)]

pub type TokenType = String;

#[derive(Clone, Debug)]
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
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const AST: &str = "*";
pub const SLASH: &str = "/";
pub const EQ: &str = "==";
pub const NEQ: &str = "!=";

pub const LT: &str = "<";
pub const GT: &str = ">";
pub const GEQ: &str = ">=";
pub const LEQ: &str = "<=";

// Deliminator
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

// Prances and Braces
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "fn";
pub const LET: &str = "let";
pub const IF: &str = "if";
pub const ELSE: &str = "else";
pub const TRUE: &str = "true";
pub const FALSE: &str = "false";
pub const RETURN: &str = "return";

pub fn empty_token() -> Token {
    Token {
        Type: "".to_string(),
        Literal: "".to_string(),
    }
}
