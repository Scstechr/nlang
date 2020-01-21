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
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut flag = true;
        while flag {
            let c = char::from(self.ch);
            if c.is_whitespace() {
                self.read_char();
            } else {
                flag = false;
            }
        }

        let tok: token::Token = match self.ch {
            b'=' => new_token(token::ASSIGN, &[self.ch]),
            b'+' => new_token(token::PLUS, &[self.ch]),
            b',' => new_token(token::COMMA, &[self.ch]),
            b';' => new_token(token::SEMICOLON, &[self.ch]),
            b'(' => new_token(token::LPAREN, &[self.ch]),
            b')' => new_token(token::RPAREN, &[self.ch]),
            b'{' => new_token(token::LBRACE, &[self.ch]),
            b'}' => new_token(token::RBRACE, &[self.ch]),
            _ => {
                if self.ch > 0 {
                    if is_letter(&self.ch) {
                        let literal = self.read_identifier();
                        let t = match literal.as_str() {
                            "fn" => token::FUNCTION.to_string(),
                            "let" => token::LET.to_string(),
                            _ => token::ID.to_string(),
                        };
                        return token::Token {
                            Type: t,
                            Literal: literal,
                        };
                    } else if is_digit(&self.ch) {
                        return token::Token {
                            Type: token::INT.to_string(),
                            Literal: self.read_number(),
                        };
                    } else {
                        new_token(token::ILLEGAL, &[self.ch])
                    }
                } else {
                    token::Token {
                        Type: token::EOF.to_string(),
                        Literal: "".to_string(),
                    }
                }
            }
        };
        self.read_char();
        return tok;
    }
}

pub fn new(input: &str) -> Lexer {
    let mut l = Lexer {
        input: input.to_string(),
        position: 0,
        read_position: 0,
        ch: 0,
    };
    l.read_char();
    return l;
}

pub fn new_token(tokenType: &'static str, ch: &[u8]) -> token::Token {
    let converted: String = String::from_utf8(ch.to_vec()).unwrap();
    token::Token {
        Type: tokenType.to_string(),
        Literal: converted,
    }
}

fn is_letter(ch: &u8) -> bool {
    let c = char::from(*ch);
    c.is_alphabetic() || c == '_'
}

fn is_digit(ch: &u8) -> bool {
    let c = char::from(*ch);
    c.is_digit(10)
}
