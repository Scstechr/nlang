extern crate nlang;
use nlang::lexer;
use nlang::token;

#[test]
fn lexer_simple_test() {
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
    for t in tests.iter() {
        let tok = l.next_token();
        assert_eq!(tok.Type, t.Type);
        assert_eq!(tok.Literal, t.Literal);
    }
}

#[test]
fn lexer_operator_test() {
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
        lexer::new_token(token::IDENT, "ten".as_bytes()),
        lexer::new_token(token::ASSIGN, "=".as_bytes()),
        lexer::new_token(token::INT, "10".as_bytes()),
        lexer::new_token(token::SEMICOLON, ";".as_bytes()),
        // let {
        lexer::new_token(token::LET, "let".as_bytes()),
        lexer::new_token(token::LBRACE, "{".as_bytes()),
        // add = x + y,
        lexer::new_token(token::IDENT, "add".as_bytes()),
        lexer::new_token(token::ASSIGN, "=".as_bytes()),
        lexer::new_token(token::IDENT, "x".as_bytes()),
        lexer::new_token(token::PLUS, "+".as_bytes()),
        lexer::new_token(token::IDENT, "y".as_bytes()),
        lexer::new_token(token::COMMA, ",".as_bytes()),
        // sub = x - y,
        lexer::new_token(token::IDENT, "sub".as_bytes()),
        lexer::new_token(token::ASSIGN, "=".as_bytes()),
        lexer::new_token(token::IDENT, "x".as_bytes()),
        lexer::new_token(token::MINUS, "-".as_bytes()),
        lexer::new_token(token::IDENT, "y".as_bytes()),
        lexer::new_token(token::COMMA, ",".as_bytes()),
        // mul = x * y,
        lexer::new_token(token::IDENT, "mul".as_bytes()),
        lexer::new_token(token::ASSIGN, "=".as_bytes()),
        lexer::new_token(token::IDENT, "x".as_bytes()),
        lexer::new_token(token::AST, "*".as_bytes()),
        lexer::new_token(token::IDENT, "y".as_bytes()),
        lexer::new_token(token::COMMA, ",".as_bytes()),
        // div = x / y,
        lexer::new_token(token::IDENT, "div".as_bytes()),
        lexer::new_token(token::ASSIGN, "=".as_bytes()),
        lexer::new_token(token::IDENT, "x".as_bytes()),
        lexer::new_token(token::SLASH, "/".as_bytes()),
        lexer::new_token(token::IDENT, "y".as_bytes()),
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
    let input = "if a {
!= b { >= c },
== d { <= e },
};";
    let tests = vec![
        // if a {
        lexer::new_token(token::IF, "if".as_bytes()),
        lexer::new_token(token::IDENT, "a".as_bytes()),
        lexer::new_token(token::LBRACE, "{".as_bytes()),
        // != b { c },
        lexer::new_token(token::NEQ, "!=".as_bytes()),
        lexer::new_token(token::IDENT, "b".as_bytes()),
        lexer::new_token(token::LBRACE, "{".as_bytes()),
        lexer::new_token(token::GEQ, ">=".as_bytes()),
        lexer::new_token(token::IDENT, "c".as_bytes()),
        lexer::new_token(token::RBRACE, "}".as_bytes()),
        lexer::new_token(token::COMMA, ",".as_bytes()),
        // } else {
        lexer::new_token(token::EQ, "==".as_bytes()),
        lexer::new_token(token::IDENT, "d".as_bytes()),
        lexer::new_token(token::LBRACE, "{".as_bytes()),
        lexer::new_token(token::LEQ, "<=".as_bytes()),
        lexer::new_token(token::IDENT, "e".as_bytes()),
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
