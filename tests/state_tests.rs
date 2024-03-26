#[cfg(test)]
mod state_tests {
    use woc_lang::{
        ast::{ast::Node, expression::IdentifierExp, statement::LetStatement},
        token::token::{Token, TokenType},
    };

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";

        let ident = IdentifierExp::new(Token::new(TokenType::Ident, "x"), "x".to_string());

        let exp = IdentifierExp::new(Token::new(TokenType::IntegerNum, "5"), "5".to_string());

        let let_state = LetStatement::new(
            Token::new(TokenType::Let, "let"),
            ident,
            Some(Box::new(exp)),
        );

        assert_eq!(let_state.to_string(), input);
    }
}
