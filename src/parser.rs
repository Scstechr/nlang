#![allow(unused_variables)]

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

    fn expect_peek(&self, t: &token::TokenType) -> bool {
        true
    }

    fn parse_statement(&self) -> ast::Statement {
        match &self.cur_token.Type as &str {
            token::LET => return ast::empty_statement(),
            _ => return ast::empty_statement(),
        }
    }

    fn parse_program(&self) -> *const ast::Program {
        let program = &ast::Program {
            Statements: vec![ast::empty_statement()],
        };
        while self.cur_token.Type != token::EOF {
            let stmt = self.parse_statement();
        }
        return ptr::null();
    }

    fn parse_let_statement(&self) -> *const ast::Statement {
        let mut stmt = &mut ast::LetStatement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: ast::Expression {},
        };
        if !self.expect_peek(&token::ID.to_string()) {
            return ptr::null();
        }
        stmt.Name = &mut ast::Identifier {
            Token: self.cur_token.clone(),
            Value: self.cur_token.Literal.clone(),
        };
        &ast::empty_statement()
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
