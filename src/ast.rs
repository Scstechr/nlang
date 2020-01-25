use crate::token;

pub struct Expression {
}

pub struct Identifier {
}

pub struct Statement {
    Token: token::Token,
    Name: Identifier,
    Value: Expression,
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


