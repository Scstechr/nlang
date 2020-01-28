use nlang::{self, ast::Statement, lexer, parser};

#[test]
fn parser_test_let_statements() {
    // let input = "let x 5;
    // let = 10;
    // let 838383;";
    let input = "let x = 5;
    let y = 10;
    let foobar = 838383;";
    let l = lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);
    // assert_eq!(3, program.Statements.len());
    for stmt in &program.Statements {
        parser_test_let_statement(&stmt);
    }
}

fn parser_test_let_statement(s: &Statement) {
    println!("{:?}", s);
    assert_eq!("let", s.token_literal());
}

fn check_parser_errors(p: &parser::Parser) {
    let errors = p.errors();
    if errors.is_empty() {
        return;
    } else {
        println!("parser has {} error(s)", errors.len());
        for m in errors {
            println!("parser error: {}", m);
        }
        panic!("exit due to parsing error...");
    }
}
