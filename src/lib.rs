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
    fn lexer_operator_test() {
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "let ten = 10;

let {
    add = x + y,
    sub = x - y,
    mul = x * y,
    div = x / y,
};";
        let tests = vec![
            // let ten = 10;
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::ID, "ten".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::INT, "10".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            // let {
            lexer::new_token(token::LET, "let".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            // add = x + y,
            lexer::new_token(token::ID, "add".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::PLUS, "+".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // sub = x - y,
            lexer::new_token(token::ID, "sub".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::MINUS, "-".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // mul = x * y,
            lexer::new_token(token::ID, "mul".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::AST, "*".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // div = x / y,
            lexer::new_token(token::ID, "div".as_bytes()),
            lexer::new_token(token::ASSIGN, "=".as_bytes()),
            lexer::new_token(token::ID, "x".as_bytes()),
            lexer::new_token(token::SLASH, "/".as_bytes()),
            lexer::new_token(token::ID, "y".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // };
            lexer::new_token(token::RBRACE, "}".as_bytes()),
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

    #[test]
    fn lexer_keyword_test() {
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "if true {
    return false;
} else {
    return true;
};
";
        let tests = vec![
            // if true {
            lexer::new_token(token::IF, "if".as_bytes()),
            lexer::new_token(token::TRUE, "true".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            // return false;
            lexer::new_token(token::RETURN, "return".as_bytes()),
            lexer::new_token(token::FALSE, "false".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            // } else {
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::ELSE, "else".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            // return true;
            lexer::new_token(token::RETURN, "return".as_bytes()),
            lexer::new_token(token::TRUE, "true".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            // };
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::EOF, "".as_bytes()),
        ];
        let mut l = lexer::new(input);
        // println!("{:#?}", l);
        for (_, t) in tests.iter().enumerate() {
            let tok = l.next_token();
            // println!("{:#?}", tok.Literal);
            // println!(
            //     "tok: [{:#?}:{:#?}]\x1b[30Gt: [{:#?}:{:#?}]",
            //     tok.Type, tok.Literal, t.Type, t.Literal
            // );
            assert_eq!(tok.Type, t.Type);
            assert_eq!(tok.Literal, t.Literal);
        }
    }

    #[test]
    fn lexer_eq_test() {
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "if a {
    != b { >= c },
    == d { <= e },
};";
        let tests = vec![
            // if a {
            lexer::new_token(token::IF, "if".as_bytes()),
            lexer::new_token(token::ID, "a".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            // != b { c },
            lexer::new_token(token::NEQ, "!=".as_bytes()),
            lexer::new_token(token::ID, "b".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            lexer::new_token(token::GEQ, ">=".as_bytes()),
            lexer::new_token(token::ID, "c".as_bytes()),
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // } else {
            lexer::new_token(token::EQ, "==".as_bytes()),
            lexer::new_token(token::ID, "d".as_bytes()),
            lexer::new_token(token::LBRACE, "{".as_bytes()),
            lexer::new_token(token::LEQ, "<=".as_bytes()),
            lexer::new_token(token::ID, "e".as_bytes()),
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::COMMA, ",".as_bytes()),
            // return true;
            lexer::new_token(token::RBRACE, "}".as_bytes()),
            lexer::new_token(token::SEMICOLON, ";".as_bytes()),
            lexer::new_token(token::EOF, "".as_bytes()),
        ];
        let mut l = lexer::new(input);
        // println!("{:#?}", l);
        for (_, t) in tests.iter().enumerate() {
            let tok = l.next_token();
            // println!("{:#?}", tok.Literal);
            // println!(
            //     "tok: [{:#?}:{:#?}]\x1b[30Gt: [{:#?}:{:#?}]",
            //     tok.Type, tok.Literal, t.Type, t.Literal
            // );
            assert_eq!(tok.Type, t.Type);
            assert_eq!(tok.Literal, t.Literal);
        }
    }
}
