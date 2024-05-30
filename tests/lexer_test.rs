#[cfg(test)]
mod lexer_test {
    use std::rc::Rc;
    use std::vec;

    use woc_lang::lexer::lexer::Lexer;
    use woc_lang::token::token::Token;
    use woc_lang::token::types::TokenType;

    #[test]
    fn test_woc_file() {
        let file_path = "woc_test_files/hello_world.woc";
        let file_content = std::fs::read_to_string(file_path).unwrap();
        for line in file_content.lines() {
            println!("{}", line);
        }
    }

    #[test]
    fn test_woc_file_tokens() {
        let lexer = Lexer::new("woc_test_files/hello_world.woc");
        let tokens_iter = lexer.tokens_iter();

        let expects = vec![
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 1, 0),
            Token::new(TokenType::Ident, "a", "woc_test_files/hello_world.woc", 1, 0),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 1, 0),
            Token::new(TokenType::IntegerNum, "1", "woc_test_files/hello_world.woc", 1, 0),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 1, 0),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 2, 0),
            Token::new(TokenType::Ident, "b", "woc_test_files/hello_world.woc", 2, 0),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 2, 0),
            Token::new(TokenType::IntegerNum, "2", "woc_test_files/hello_world.woc", 2, 0),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 2, 0),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 3, 0),
            Token::new(TokenType::Ident, "c", "woc_test_files/hello_world.woc", 3, 0),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 3, 0),
            Token::new(TokenType::IntegerNum, "3", "woc_test_files/hello_world.woc", 3, 0),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 3, 0),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Ident, "arr", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::LeftBracket, "[", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Ident, "a", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Comma, ",", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Ident, "b", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Comma, ",", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Ident, "c", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::RightBracket, "]", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 4, 0),
            Token::new(TokenType::Eof, "", "woc_test_files/hello_world.woc", 4, 0),
        ];

        for expect in expects {
            let real = tokens_iter.next().unwrap();
            test_equal_tokens(expect, real);
        }
    }

    fn test_equal_tokens(expect: Token, real: Rc<Token>) {
        assert_eq!(expect.token_type(), real.token_type());
        assert_eq!(expect.literal(), real.literal());
        assert_eq!(expect.file_path(), real.file_path());
        assert_eq!(expect.file_row_number(), real.file_row_number());
    }
}
