#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    fn read_char(l: &mut Lexer) {
        if l.read_position >= l.input.len() {
            l.ch = 0;
        } else {
            l.ch = l.input.as_bytes()[l.read_position];
        }
        l.position = l.read_position;
        l.read_position += 1;
    }
}
