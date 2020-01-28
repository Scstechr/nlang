#![allow(unused_variables)]

use crate::{ast, lexer, token::{self, Token}};
use std::ptr;

pub struct Parser {
    l: lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_statement(&self) -> (ast::Statement, bool) {
        match &self.cur_token.Type as &str {
            token::LET => return (ast::empty_statement(), true),
            _ => return (ast::empty_statement(), false),
        }
    }

    pub fn parse_program(&mut self) -> *mut ast::Program {
        let program = &mut ast::Program { Statements: vec![] };
        while self.cur_token.Type != token::EOF {
            let (stmt, f) = self.parse_statement();
            if f {
                program.Statements.push(stmt);
            }
            self.next_token();
        }
        return program;
    }

    fn parse_let_statement(&mut self) -> *mut ast::LetStatement {
        let mut stmt = &mut ast::LetStatement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: ast::Expression {},
        };
        if !self.expect_peek(token::ID) {
            return ptr::null_mut();
        }
        stmt.Name = &mut ast::Identifier {
            Token: self.cur_token.clone(),
            Value: self.cur_token.Literal.clone(),
        };
        if !self.expect_peek(token::ASSIGN) {
            return ptr::null_mut();
        }
        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }
        return stmt;
    }

    fn peek_token_is(&self, t: &'static str) -> bool {
        self.peek_token.Type == t
    }

    fn cur_token_is(&self, t: &'static str) -> bool {
        self.peek_token.Type == t
    }

    fn expect_peek(&mut self, t: &'static str) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
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
