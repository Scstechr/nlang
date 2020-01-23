use crate::{lexer, token};
use std::io::{stdin, stdout, Write};

const PROMPT: &str = ">> ";

fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("-a");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

pub fn start() {
    loop {
        print!("{}", PROMPT);
        let mut l = lexer::new(&input());
        loop {
            print!("\x1b[90m");
            let tok = l.next_token();
            println!("[Type:{:#?}, Literal:{:#?}]", tok.Type, tok.Literal);
            if tok.Type == "EOF" {
                print!("\x1b[m");
                break;
            }
        }
    }
}
