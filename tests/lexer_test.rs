mod lexer_test {
    use woc_lang::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

    #[test]
    fn test_ten_token() {
        let input = " ten  &&& ";

        let l = Lexer::new(input);

        let tokens = vec![
            Token::new(TokenType::Literal, "ten"),
            Token::new(TokenType::And, "&&"),
            Token::new(TokenType::BitAnd, "&"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_integer_and_float_num() {
        let input: &str = "10 1_000 1.0 1_000.0 99.99";

        let l = Lexer::new(input);

        let tokens = vec![
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::Num, "1_000"),
            Token::new(TokenType::Float, "1.0"),
            Token::new(TokenType::Float, "1_000.0"),
            Token::new(TokenType::Float, "99.99"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_minus_number() {
        let input: &str = "-10 -1_000 -1.0 -1_000.0 -99.99";

        let l = Lexer::new(input);

        let tokens = vec![
            Token::new(TokenType::Num, "-10"),
            Token::new(TokenType::Num, "-1_000"),
            Token::new(TokenType::Float, "-1.0"),
            Token::new(TokenType::Float, "-1_000.0"),
            Token::new(TokenType::Float, "-99.99"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_if_token() {
        let input = "
            if (5 < 10) {
                return true;
            ";

        let l = Lexer::new(input);

        let tokens = vec![
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Less, "<"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }

    #[test]
    fn test_next_token() {
        let input = "
            let five = 5;
            let ten = 10;
            func (x, y) {
                x + y;
            }
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
            10 == 10;
            10 != 9;
            for (let i = 0; i < 10; i += 1) {
                i;
            }
        ";

        let l = Lexer::new(input);

        let tokens = vec![
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Literal, "five"),
            Token::new(TokenType::Assignment, "="),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Literal, "ten"),
            Token::new(TokenType::Assignment, "="),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Func, "func"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Literal, "x"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Literal, "y"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Literal, "x"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Literal, "y"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Literal, "result"),
            Token::new(TokenType::Assignment, "="),
            Token::new(TokenType::Literal, "add"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Literal, "five"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Literal, "ten"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Not, "!"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::Star, "*"),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Less, "<"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::Greater, ">"),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Num, "5"),
            Token::new(TokenType::Less, "<"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::EqualTo, "=="),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::NotEqualTo, "!="),
            Token::new(TokenType::Num, "9"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::For, "for"),
            Token::new(TokenType::LeftParen, "("),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Literal, "i"),
            Token::new(TokenType::Assignment, "="),
            Token::new(TokenType::Num, "0"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Literal, "i"),
            Token::new(TokenType::Less, "<"),
            Token::new(TokenType::Num, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Literal, "i"),
            Token::new(TokenType::PlusAssign, "+="),
            Token::new(TokenType::Num, "1"),
            Token::new(TokenType::RightParen, ")"),
            Token::new(TokenType::LeftBrace, "{"),
            Token::new(TokenType::Literal, "i"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RightBrace, "}"),
            Token::new(TokenType::Eof, ""),
        ];

        for token in tokens.iter() {
            let next_token = l.next_token().unwrap();
            assert_eq!(*token.token_type(), *next_token.token_type());
            assert_eq!(token.literal(), next_token.literal());
        }
    }
}
