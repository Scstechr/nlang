use crate::token;

#[derive(Debug, PartialEq)]
pub struct Expression {}

impl Expression {
    fn expression_node(&self) {}
    pub fn string(&self) -> String {
        "".to_string()
    }
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
    pub fn string(&self) -> String {
        // println!("{:#?}", self);
        match &self.token_literal() as &str {
            token::LET => {
                return format!(
                    "{t:?} {n:?} = {v:?};",
                    t = self.token_literal(),
                    n = self.Name.string(),
                    v = self.Value,
                )
            }
            token::RETURN => {
                return format!("{t:?} {v:?};", t = self.token_literal(), v = self.Value,)
            }
            _ => {
                return format!(
                    "{t:?} {n:?} = {v:?};",
                    t = self.token_literal(),
                    n = self.Name.string(),
                    v = self.Value,
                )
            }
        }
        // match self.token_literal()
        // let out = if self.token_literal format!(
        //     "{t:?} {n:?} = {v:?};",
        //     t = self.token_literal(),
        //     n = self.Name.string(),
        //     v = self.Value,
        // );
        // // if self.Value != {} {
        // //     concat!(out, self.Value.string());
        // // }

        // return out.to_string();
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

    pub fn string(&self) -> String {
        let mut out = Vec::new();
        for s in &self.Statements {
            out.push(s.string());
        }
        out.concat()
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
    pub fn string(&self) -> String {
        self.Value.clone()
    }
}

pub fn empty_identifier() -> Identifier {
    Identifier {
        Token: token::empty_token(),
        Value: "".to_string(),
    }
}

pub fn empty_statement() -> Statement {
    Statement {
        Token: token::empty_token(),
        Name: empty_identifier(),
        Value: Expression {},
    }
}

// #[derive(Debug)]
// pub struct ExpressionStatement {
//     pub Token: token::Token,
//     pub Expression: Expression,
// }

// impl ExpressionStatement {
//     fn expression_node(&self) {}
//     pub fn token_literal(&self) -> &String {
//         &self.Token.Literal
//     }
//     pub fn string(&self) -> String {
//         format!("{}", self.Expression.string())
//     }
// }
