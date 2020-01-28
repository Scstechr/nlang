extern crate nlang;
use nlang::{ast::Statement, lexer, parser};
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
    parser_test_let_statement(&program.Statements);
    // println!("{:#?}", program);
    // unsafe {
    //     if let Some(ptr) = p.as_ref() {
    //         let program = ptr.parse_program();
    //     }
    //     // let program = p.as_ref().unwrap().parse_program();
    // }
}

fn parser_test_let_statement(s: &Vec<Statement>) {
    for stmt in s {
        println!("{:?}", stmt);
        if stmt.token_literal() != "let" {
            println!("error");
        }
    }
}
// Program {
//     Statements: [
//         Statement {
//             Token: Token {
//                 Type: "let",
//                 Literal: "let",
//             },
//             Name: Identifier {
//                 Token: Token {
//                     Type: "ID",
//                     Literal: "x",
//                 },
//                 Value: "x",
//             },
//             Value: Expression,
//         },
//         Statement {
//             Token: Token {
//                 Type: "let",
//                 Literal: "let",
//             },
//             Name: Identifier {
//                 Token: Token {
//                     Type: "ID",
//                     Literal: "y",
//                 },
//                 Value: "y",
//             },
//             Value: Expression,
//         },
//         Statement {
//             Token: Token {
//                 Type: "let",
//                 Literal: "let",
//             },
//             Name: Identifier {
//                 Token: Token {
//                     Type: "ID",
//                     Literal: "foobar",
//                 },
//                 Value: "foobar",
//             },
//             Value: Expression,
//         },
//     ],
