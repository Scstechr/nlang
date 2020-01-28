use crate::token;

pub struct Expression {}

pub struct Identifier {}

pub struct LetStatement {
    Token: token::Token,
    Name: Identifier,
    Value: Expression,
}

impl LetStatement {
    fn statementNode(&self) {}
    fn TokenLiteral(&self) -> &String {
        &self.Token.Literal
    }
}
// pub trait Node {
//     fn token_literal() -> String;
// }

// pub struct Statement {
// }

// impl Statement {
//     fn statement_node(&self);
// }

// pub struct Program {
//     Statement: Vec<Statement>,
// }
