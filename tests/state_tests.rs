mod state_tests {
    use woc_lang::{
        ast::{expression::IdentifierExp, statement::LetState, Node},
        token::{Token, TokenType},
    };

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";

        let ident = IdentifierExp::new(Token::new(TokenType::Literal, "x"), "x".to_string());

        let exp = IdentifierExp::new(Token::new(TokenType::Num, "5"), "5".to_string());

        let let_state = LetState::new(Token::new(TokenType::Let, "let"), ident, Box::new(exp));

        assert_eq!(let_state.to_string(), input);
    }
}
