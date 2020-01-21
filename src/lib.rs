#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {

    #[test]
    fn lexer_test() {
        // println!("# lexer_test");
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let input = "=+(){},;";
        // let five = 5;
        // let ten = 10;

        // let add = fn(x, y) {
        //     x + y;
        // };

        // let result = add(five, ten);
        // """;
        let tests = vec![
            token::Token {
                Type: token::ASSIGN.to_string(),
                Literal: "=".as_bytes()[0],
            },
            token::Token {
                Type: token::PLUS.to_string(),
                Literal: "+".as_bytes()[0],
            },
            token::Token {
                Type: token::LPAREN.to_string(),
                Literal: "(".as_bytes()[0],
            },
            token::Token {
                Type: token::RPAREN.to_string(),
                Literal: ")".as_bytes()[0],
            },
            token::Token {
                Type: token::LBRACE.to_string(),
                Literal: "{".as_bytes()[0],
            },
            token::Token {
                Type: token::RBRACE.to_string(),
                Literal: "}".as_bytes()[0],
            },
            token::Token {
                Type: token::COMMA.to_string(),
                Literal: ",".as_bytes()[0],
            },
            token::Token {
                Type: token::SEMICOLON.to_string(),
                Literal: ";".as_bytes()[0],
            },
            token::Token {
                Type: token::EOF.to_string(),
                Literal: 0,
            },
        ];
        let mut l = lexer::new(input);
        // println!("{:#?}", l);
        for (_, t) in tests.iter().enumerate() {
            let tok = l.next_token();

            //             if tok.Type != t.Type {
            //                 panic!(
            //                     "tests[{}] - tokentype wrong. expected: \"{}\", got: \"{}\"",
            //                     i, t.Type, tok.Type
            //                 );
            //             }
            //             if tok.Literal != t.Literal {
            //                 let t_bytes = &[t.Literal];
            //                 let t_string: String = String::from_utf8(t_bytes.to_vec()).unwrap();
            //                 let tok_bytes = &[tok.Literal];
            //                 let tok_string: String = String::from_utf8(tok_bytes.to_vec()).unwrap();
            //                 panic!(
            //                     "tests[{}] - literal wrong. expected: \"{}\", got: \"{}\"",
            //                     i, t_string, tok_string
            //                 );
            //             }
            assert_eq!(tok.Type, t.Type);
            assert_eq!(tok.Literal, t.Literal);
        }
        // assert_eq!(input, "=+(){},;");
    }
}
