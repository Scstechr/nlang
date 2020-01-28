use crate::token;

pub struct Expression {}

impl Expression {
    fn expression_node(&self) {}
}

pub struct Statement {
    Token: token::Token,
    Name: *mut Identifier,
    Value: Expression,
}

impl Statement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

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

pub struct Identifier {
    pub Token: token::Token,
    pub Value: String,
}

impl Identifier {
    fn expression_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

pub struct LetStatement {
    pub Token: token::Token,
    pub Name: *mut Identifier,
    pub Value: Expression,
}

impl LetStatement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

pub fn empty_identifier() -> *mut Identifier {
    &mut Identifier {
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
