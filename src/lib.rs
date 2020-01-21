#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod lexer;
pub mod token;

// use lexer::Lexer;
// use token::Token;
// use token::TokenType;

#[cfg(test)]
mod tests {

    #[test]
    fn lexer_test() {
        use crate::lexer;
        // let expected_type = TokenType;
        // println!("{:?}", expected_type);
        let input = "=+(){},;";
        let l = lexer::new(input);
        assert_eq!(input, "=+(){},;");
    }
}
