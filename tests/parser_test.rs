extern crate nlang;
use nlang::{lexer, parser};
use std::ptr;

#[test]
fn parser_test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;";
    let l = lexer::new(input);
    let mut p = parser::Parser::new(l);
    println!("{:#?}", p);
    let program = p.parse_program();
    println!("{:#?}", program);
    // unsafe {
    //     if let Some(ptr) = p.as_ref() {
    //         let program = ptr.parse_program();
    //     }
    //     // let program = p.as_ref().unwrap().parse_program();
    // }
}

fn parser_test_let_statement() {}
