use crate::token;

pub struct Expression {}

impl Expression {
    fn expression_node(&self) {}
}

pub struct Statement {
    Token: token::Token,
    Name: Identifier,
    Value: Expression,
}

impl Statement {
    fn statement_node(&self) {}
    fn token_literal(&self) -> &String {
        &self.Token.Literal
    }
}

pub struct Program {
    Statements: Vec<Statement>,
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

pub fn empty_identifier() -> Identifier {
    Identifier {
        Token: token::empty_token(),
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
