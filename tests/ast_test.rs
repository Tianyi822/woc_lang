#[cfg(test)]
mod ast_test {
    use woc_lang::ast::ast::{Node, Program};
    use woc_lang::ast::expression::IdentifierExp;
    use woc_lang::ast::statement::LetStatement;
    use woc_lang::token::token::{Token, TokenType};

    #[test]
    fn test_ast() {
        let let_stat = LetStatement::new(
            Token::new(TokenType::Let, "let"),
            IdentifierExp::new(Token::new(TokenType::Ident, "myVar"), "myVar".to_string()),
            Some(Box::new(IdentifierExp::new(
                Token::new(TokenType::Ident, "anotherVar"),
                "anotherVar".to_string(),
            ))),
        );

        let mut program = Program::new();
        program.push(Box::new(let_stat));

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }
}
