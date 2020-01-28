use nlang::{self, ast::Statement, lexer, parser};
use std::ptr;

#[test]
fn parser_test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;";
    let l = lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    assert_eq!(3, program.Statements.len());
    for stmt in &program.Statements {
        parser_test_let_statement(&stmt);
    }
}

fn parser_test_let_statement(s: &Statement) {
    println!("{:?}", s);
    assert_eq!("let", s.token_literal());
}
