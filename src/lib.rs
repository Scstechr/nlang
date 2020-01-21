#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {

    #[test]
    fn lexer_simple_test() {
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "=+(){},;";
        let tests = vec![
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::PLUS, "+".as_bytes()),
            lexer::new_token(token::LPAREN, "(".as_bytes()),
            lexer::new_token(token::RPAREN, ")".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::EOF, "".as_bytes()),
        ];
        let mut l = lexer::new(input);
        for (_, t) in tests.iter().enumerate() {
            let tok = l.next_token();
            assert_eq!(tok.Type, t.Type);
            assert_eq!(tok.Literal, t.Literal);
        }
    }

    #[test]
    fn lexer_test() {
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "let five = 5;
let ten = 10;

let add_ = fn(x, y) {
    x + y;
};

let result = add(five, ten);
";
        let tests = vec![
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::ID, "five".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::INT, "5".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::ID, "ten".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::INT, "10".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::ID, "add_".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::FUNCTION, "fn".as_bytes()),
            lexer::new_token(token::LPAREN, "(".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::RPAREN, ")".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::PLUS, "+".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::ID, "result".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::ID, "add".as_bytes()),
            lexer::new_token(token::LPAREN, "(".as_bytes()),
            lexer::new_token(token::ID, "five".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            lexer::new_token(token::ID, "ten".as_bytes()),
            lexer::new_token(token::RPAREN, ")".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::EOF, "".as_bytes()),
        ];
        let mut l = lexer::new(input);
        // println!("{:#?}", l);
        for (_, t) in tests.iter().enumerate() {
            let tok = l.next_token();
            // // println!("{:#?}", tok.Literal);
            // println!(
            //     "tok: [{:#?}:{:#?}]\x1b[30Gt: [{:#?}:{:#?}]",
            //     tok.Type, tok.Literal, t.Type, t.Literal
            // );
            assert_eq!(tok.Type, t.Type);
            assert_eq!(tok.Literal, t.Literal);
        }
    }
}
