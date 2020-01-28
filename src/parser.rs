#![allow(unused_variables)]

#[derive(Debug)]
enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

use crate::{
    ast::{self, Expression, Identifier, Program, Statement},
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

    fn parse_statement(&mut self) -> (Statement, bool) {
        match &self.cur_token.Type as &str {
            token::LET => return self.parse_let_statement(),
            token::RETURN => return self.parse_return_statement(),
            _ => return self.parse_expression_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> (Statement, bool) {
        let stmt = Statement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: Expression {},
        };
        self.next_token();
        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }
        self.next_token();
        return (stmt, true);
    }

    fn parse_let_statement(&mut self) -> (Statement, bool) {
        let mut stmt = Statement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: Expression {},
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
        self.next_token();
        return (stmt, true);
    }

    fn parse_expression_statement(&mut self) -> (Statement, bool) {
        let mut stmt = Statement {
            Token: self.cur_token.clone(),
            Name: ast::empty_identifier(),
            Value: Expression {},
        };
        let (e, f) = self.parse_expression(Precedence::LOWEST);
        stmt.Value = e;
        // println!("{:?}", stmt.Value);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        return (stmt, f);
    }

    fn parse_expression(&self, precedence: Precedence) -> (Expression, bool) {
        // let (prefix, f) = self.parse_prefix(self.cur_token.Type);
        let (prefix, f) = self.parse_prefix();
        if !f {
            return (Expression {}, false);
        }
        let left_exp = prefix;
        return (left_exp, true);
    }

    fn parse_prefix(&self) -> (Expression, bool) {
        (Expression {}, true)
    }

    fn parse_infix(&self) -> (Expression, bool) {
        (Expression {}, true)
    }

    fn parse_identifier(&self) -> Identifier {
        Identifier {
            Token: self.cur_token.clone(),
            Value: self.cur_token.Literal.clone(),
        }
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
            self.peek_error(t);
            return false;
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, t: &'static str) {
        let msg = format!(
            "expected next token to be {}, got {} instead",
            t, self.peek_token.Type
        );
        self.errors.push(msg);
    }
}
