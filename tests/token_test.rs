#[cfg(test)]
mod token_test {
    use woc_lang::token::{token::Token, types::TokenType};

    #[test]
    fn test_new() {
        let token = Token::new(TokenType::For, "for", "", 0, 0);
        assert_eq!(token.token_type(), &TokenType::For);
        assert_eq!(token.literal(), "for");
    }

    #[test]
    fn test_is_eof() {
        let token = Token::new(TokenType::Eof, "", "", 0, 0);
        assert_eq!(token.is_eof(), true);
    }

    #[test]
    fn test_priority() {
        let token = Token::new(TokenType::Or, "or", "", 0, 0);
        assert_eq!(token.precedence(), 1);
        let token = Token::new(TokenType::And, "and", "", 0, 0);
        assert_eq!(token.precedence(), 2);
    }

    #[test]
    fn test_float() {
        let token = Token::new(TokenType::FloatNum, "1.0", "", 0, 0);
        assert_eq!(token.token_type(), &TokenType::FloatNum);
        assert_eq!(token.literal(), "1.0");
    }
}
