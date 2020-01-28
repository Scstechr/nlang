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
    let tests = vec!["x", "y", "foobar"];
    // assert_eq!(3, program.Statements.len());
    for (stmt, t) in program.Statements.iter().zip(tests) {
        parser_test_let_statement(&stmt, &t);
    }
}

fn parser_test_let_statement(s: &Statement, t: &str) {
    // println!("{:?}", s);
    assert_eq!("let", s.token_literal());
    assert_eq!(t, s.Name.Value);
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
