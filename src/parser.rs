use crate::{ast, lexer, token};
use std::ptr;

pub struct Parser {
    l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_program(&mut self) -> *const ast::Program {
        let program = &ast::Program {
            Statements: vec![ast::empty_statement()],
        };
        // let program = &ast::Program {
        //     Statements: vec![
        //     ast::Statement {
        //         Token: token::Token {
        //     Type: "".to_string(),
        //     Literal: "".to_string(),
        //         },
        //     Name: ast::Identifier {
        //         Token: token::Token {
        //     Type: "".to_string(),
        //     Literal: "".to_string(),
        //         }
        //         Value: ast::Expression {},
        //     }
        //     Value: ast::Expression{},
        //     }],
        // };
        return ptr::null();
    }
}

pub fn new(l: lexer::Lexer) -> *mut Parser {
    let p: &mut Parser = &mut Parser {
        l: l,
        cur_token: lexer::empty_token(),
        peek_token: lexer::empty_token(),
    };
    p.next_token();
    p.next_token();
    return p;
}
