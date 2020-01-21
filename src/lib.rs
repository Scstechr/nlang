#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {

    #[test]
    fn lexer_test() {
        println!("# lexer_test");
        use crate::lexer;
        use crate::token;
        use std::collections::HashMap;

        let mut tests: Vec<HashMap<token::TokenType, String>> = Vec::new();
        // let expected_type = TokenType;
        // println!("{:?}", expected_type);
        let input = "=+(){},;";
        let l = lexer::new(input);
        println!("{:?}", l);
        assert_eq!(input, "=+(){},;");
    }
}
