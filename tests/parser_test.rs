extern crate nlang;
use nlang::lexer;

#[test]
fn parser_test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;";
    let mut l = lexer::new(input);
}

#[test]
fn parser_test_let_statement() {}
