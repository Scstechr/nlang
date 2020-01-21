#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::token;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> token::Token {
        let tok = {
            if self.ch == b'=' {
                new_token(token::ASSIGN, self.ch)
            } else if self.ch == b'+' {
                new_token(token::PLUS, self.ch)
            } else if self.ch == b',' {
                new_token(token::COMMA, self.ch)
            } else if self.ch == b';' {
                new_token(token::SEMICOLON, self.ch)
            } else if self.ch == b'(' {
                new_token(token::LPAREN, self.ch)
            } else if self.ch == b')' {
                new_token(token::RPAREN, self.ch)
            } else if self.ch == b'{' {
                new_token(token::LBRACE, self.ch)
            } else if self.ch == b'}' {
                new_token(token::RBRACE, self.ch)
            } else {
                token::Token {
                    Type: token::EOF.to_string(),
                    Literal: "".to_string(),
                }
            }
        };
        self.read_char();
        return tok;
    }
}

fn new_token(tokenType: &'static str, ch: u8) -> token::Token {
    token::Token {
        Type: tokenType.to_string(),
        Literal: ch.to_string(),
    }
}
