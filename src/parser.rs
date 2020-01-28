#![allow(unused_variables)]

use crate::{
    ast::{self, Program, Statement},
    lexer::{self, Lexer},
    token::{self, Token},
};
use std::ptr;

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Parser {
            l: l,
            cur_token: lexer::empty_token(),
            peek_token: lexer::empty_token(),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    fn parse_statement(&mut self) -> (Statement, bool) {
        match &self.cur_token.Type as &str {
            token::LET => return self.parse_let_statement(),
            _ => return (ast::empty_statement(), false),
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            Statements: Vec::new(),
        };
        while self.cur_token.Type != token::EOF {
            let (stmt, f) = self.parse_statement();
            if f {
                program.Statements.push(stmt);
            }
            self.next_token();
        }
        return program;
    }

    fn parse_let_statement(&mut self) -> (Statement, bool) {
        let mut stmt = Statement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: ast::Expression {},
        };
        if !self.expect_peek(token::IDENT) {
            return (stmt, false);
        }
        stmt.Name = ast::Identifier {
            Token: self.cur_token.clone(),
            Value: self.cur_token.Literal.clone(),
        };
        if !self.expect_peek(token::ASSIGN) {
            return (stmt, false);
        }
        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }
        return (stmt, true);
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
