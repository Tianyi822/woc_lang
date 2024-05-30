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

        // let a = 1;
        // let b = 2;
        // let c = 3;
        // let arr = [a, b, c];
        let expects = vec![
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 1, 1),
            Token::new(TokenType::Ident, "a", "woc_test_files/hello_world.woc", 1, 2),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 1, 3),
            Token::new(TokenType::IntegerNum, "1", "woc_test_files/hello_world.woc", 1, 4),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 1, 5),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 2, 1),
            Token::new(TokenType::Ident, "b", "woc_test_files/hello_world.woc", 2, 2),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 2, 3),
            Token::new(TokenType::IntegerNum, "2", "woc_test_files/hello_world.woc", 2, 4),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 2, 5),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 3, 1),
            Token::new(TokenType::Ident, "c", "woc_test_files/hello_world.woc", 3, 2),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 3, 3),
            Token::new(TokenType::IntegerNum, "3", "woc_test_files/hello_world.woc", 3, 4),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 3, 5),
            Token::new(TokenType::Let, "let", "woc_test_files/hello_world.woc", 4, 1),
            Token::new(TokenType::Ident, "arr", "woc_test_files/hello_world.woc", 4, 2),
            Token::new(TokenType::Assignment, "=", "woc_test_files/hello_world.woc", 4, 3),
            Token::new(TokenType::LeftBracket, "[", "woc_test_files/hello_world.woc", 4, 4),
            Token::new(TokenType::Ident, "a", "woc_test_files/hello_world.woc", 4, 5),
            Token::new(TokenType::Comma, ",", "woc_test_files/hello_world.woc", 4, 6),
            Token::new(TokenType::Ident, "b", "woc_test_files/hello_world.woc", 4, 7),
            Token::new(TokenType::Comma, ",", "woc_test_files/hello_world.woc", 4, 8),
            Token::new(TokenType::Ident, "c", "woc_test_files/hello_world.woc", 4, 9),
            Token::new(TokenType::RightBracket, "]", "woc_test_files/hello_world.woc", 4, 10),
            Token::new(TokenType::Semicolon, ";", "woc_test_files/hello_world.woc", 4, 11),
            Token::new(TokenType::Eof, "", "woc_test_files/hello_world.woc", 5, 1),
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
        assert_eq!(expect.token_number_in_line(), real.token_number_in_line());
    }
}
