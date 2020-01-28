use crate::token;

pub struct Expression {}

impl Expression {
    fn expression_node(&self) {}
}

pub struct Statement {}

impl Statement {
    fn statement_node(&self) {}
}

pub struct Program {
    Statements: Vec<Statement>,
}

pub struct Identifier {
    Token: token::Token,
    Value: Expression,
}

impl Identifier {
    fn expression_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

pub struct LetStatement {
    Token: token::Token,
    Name: Identifier,
    Value: Expression,
}

impl LetStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

// pub trait Node {
//     fn token_literal() -> String;
// }

// impl Statement {
//     fn statement_node(&self);
// }

// pub struct Program {
//     Statement: Vec<Statement>,
// }
