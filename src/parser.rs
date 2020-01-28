use crate::{ast, lexer, token};

pub struct Parser {
    l: *mut lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}
