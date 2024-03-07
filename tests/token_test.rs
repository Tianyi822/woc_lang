mod token_test {
    use woc_lang::token::{Token, TokenType};

    #[test]
    fn test_new() {
        let token = Token::new(TokenType::For, "for");
        assert_eq!(token.token_type(), &TokenType::For);
        assert_eq!(token.literal(), "for");
    }

    #[test]
    fn test_is_eof() {
        let token = Token::new(TokenType::Eof, "");
        assert_eq!(token.is_eof(), true);
    }

    #[test]
    fn test_priority() {
        let token = Token::new(TokenType::Or, "or");
        assert_eq!(token.priority(), 1);
        let token = Token::new(TokenType::And, "and");
        assert_eq!(token.priority(), 2);
    }

    #[test]
    fn test_float() {
        let token = Token::new(TokenType::Float, "1.0");
        assert_eq!(token.token_type(), &TokenType::Float);
        assert_eq!(token.literal(), "1.0");
    }
}
