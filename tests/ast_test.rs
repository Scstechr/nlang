use nlang::{
    self,
    ast::{Expression, Identifier, Program, Statement},
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
            Value: Expression {},
        }],
    };
    assert_eq!("\"let\" \"myVar\" = Expression;", program.string());
}
