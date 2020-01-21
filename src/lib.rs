#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod lexer;
pub mod token;

use token::TokenType;

#[cfg(test)]
mod tests {

    #[test]
    fn lexer_test() {
        // let expected_type = TokenType;
        // println!("{:?}", expected_type);
        let input = "=+(){},;";
        assert_eq!(input, "=+(){},;");
    }
}
