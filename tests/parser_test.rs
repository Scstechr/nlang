extern crate nlang;
use nlang::{lexer, parser};

#[test]
fn parser_test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;";
    let l = lexer::new(input);
    let p = parser::new(l);
    let program = p.parse_program();
}

#[test]
fn parser_test_let_statement() {}
