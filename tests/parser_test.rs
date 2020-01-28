extern crate nlang;
use nlang::{lexer, parser};
use std::ptr;

#[test]
fn parser_test_let_statements() {
    let input = "let x = 5;
let y = 10;
let foobar = 838383;";
    let l = lexer::new(input);
    let p = parser::new(l);
    unsafe {
        let program = (&mut *p).parse_program();
        if program == ptr::null_mut() {
            panic!("parse_program returned null()");
        }
    }
    // unsafe {
    //     if let Some(ptr) = p.as_ref() {
    //         let program = ptr.parse_program();
    //     }
    //     // let program = p.as_ref().unwrap().parse_program();
    // }
}

#[test]
fn parser_test_let_statement() {}
