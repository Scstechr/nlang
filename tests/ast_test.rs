use nlang::{
    self,
    ast::{self, Expression, Identifier, Program, Statement},
    lexer, parser,
    token::{self, Token},
};

#[test]
fn ast_test() {
    let program = &Program {
        Statements: vec![Statement {
            Token: Token {
                Type: token::LET.to_string(),
                Literal: "let".to_string(),
            },
            Name: Identifier {
                Token: Token {
                    Type: token::IDENT.to_string(),
                    Literal: "myVar".to_string(),
                },
                Value: "myVar".to_string(),
            },
            Value: Expression {
                Name: ast::empty_identifier(),
            },
        }],
    };
    assert_eq!("\"let\" \"myVar\" = Expression { Name: Identifier { Token: Token { Type: \"\", Literal: \"\" }, Value: \"\" } };", program.string());
}

#[test]
fn ast_simple_test() {
    let input = "foobar;";
    let l = lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    check_parser_errors(&p);
    assert_eq!(1, program.Statements.len());
    println!("{:#?}", program);
    assert_eq!("IDENT", program.Statements[0].Value.token_literal());
    // for (stmt, t) in program.Statements.iter().zip(tests) {
    //     parser_test_let_statement(&stmt, &t);
    // }
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
