use crate::token;

#[derive(Debug)]
pub struct Expression {}

impl Expression {
    fn expression_node(&self) {}
}

#[derive(Debug)]
pub struct Statement {
    pub Token: token::Token,
    pub Name: Identifier,
    pub Value: Expression,
}

impl Statement {
    fn statement_node(&self) {}
    pub fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

#[derive(Debug)]
pub struct Program {
    pub Statements: Vec<Statement>,
}

impl Program {
    fn token_literal(&self) -> String {
        if &self.Statements.len() > &0 {
            (&self.Statements[0].token_literal()).to_string()
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub Token: token::Token,
    pub Value: String,
}

impl Identifier {
    fn expression_node(&self) {}
    pub fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

pub fn empty_identifier() -> Identifier {
    Identifier {
        Token: token::empty_token(),
        Value: "EMPTY".to_string(),
    }
}

pub fn empty_statement() -> Statement {
    Statement {
        Token: token::empty_token(),
        Name: empty_identifier(),
        Value: Expression {},
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
