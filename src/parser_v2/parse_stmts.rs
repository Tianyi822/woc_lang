use crate::{
    ast_v2::{expressions::IdentifierExp, statements::LetStatement},
    token::{precedence::*, token::TokenType},
};
use crate::ast_v2::statements::ReturnStatement;

use super::parser::Parser;

impl Parser {
    pub(super) fn parse_let_stmt(&self) -> Option<LetStatement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        // Move to Ident token
        let name = self.get_cur_token().literal().to_string();
        // Create IdentifierExp
        let ident_exp = IdentifierExp::new(name);

        // Move to Assign token
        if !self.expect_peek(&TokenType::Assignment) {
            return None;
        }
        // Skip Assign token and move to the Expression token.
        self.next_token();

        // Parse the expression
        let exp = self.parse_expression(LEVEL_0);

        if !self.expect_peek(&TokenType::Semicolon) {
            self.store_error(&format!(
                "Expected next token to be Semicolon, got {:?} instead.",
                self.get_cur_token().token_type()
            ));
        }

        Some(LetStatement::new(ident_exp, exp))
    }

    pub(super) fn parse_return_stmt(&self) -> Option<ReturnStatement> {
        // Move to the next token
        self.next_token();
        // Parse the expression
        let exp = self.parse_expression(LEVEL_0);
        if !self.expect_peek(&TokenType::Semicolon) {
            self.store_error(&format!(
                "Expected next token to be Semicolon, got {:?} instead.",
                self.get_cur_token().token_type()
            ));
        }

        Some(ReturnStatement::new(exp))
    }
}
