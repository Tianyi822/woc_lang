use crate::ast::expression::IdentifierExp;
use crate::ast::Statement;
use crate::ast::statement::{LetStatement, ReturnStatement};
use crate::parser::Parser;
use crate::token::TokenType;

use super::LEVEL_0;

impl Parser {
    // This method is used to parse the let statement.
    pub(super) fn parse_let_statement(&self) -> Option<Box<dyn Statement>> {
        let let_tok = self.cur_token.borrow().clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        // Build the identifier expression.
        let cur_tok = self.get_cur_token();
        let value = cur_tok.literal().to_string();
        let ident = IdentifierExp::new(
            cur_tok,
            value,
        );

        // Check the next token is an assignment operator,
        if self.expect_peek(TokenType::Assignment) {
            // Skip the assignment operator.
            self.next_token()
        } else {
            return None;
        }

        let value = self.parse_expression(LEVEL_0);

        while !self.cur_tok_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(LetStatement::new(let_tok, ident, value)))
    }

    // This method is used to parse the return statement.
    pub(super) fn parse_return_statement(&self) -> Option<Box<dyn Statement>> {
        let return_tok = self.get_cur_token();

        self.next_token();

        let value = self.parse_expression(LEVEL_0);

        while !self.cur_tok_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement::new(return_tok, value)))
    }
}