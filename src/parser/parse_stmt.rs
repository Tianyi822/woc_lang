use crate::ast::expression::IdentifierExp;
use crate::ast::Statement;
use crate::ast::statement::{LetStatement, ReturnStatement};
use crate::parser::Parser;
use crate::token::TokenType;

impl Parser {
    // This method is used to parse the let statement.
    pub(super) fn parse_let_statement(&self) -> Option<Box<dyn Statement>> {
        let let_tok = self.cur_token.borrow().clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let ident = IdentifierExp::new(
            self.cur_token.borrow().clone(),
            self.cur_token.borrow().literal().to_string(),
        );

        // Check the next token is an assignment operator,
        if !self.expect_peek(TokenType::Assignment) {
            return None;
        }

        while self.cur_tok_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(LetStatement::new(let_tok, ident, None)))
    }

    // This method is used to parse the return statement.
    pub(super) fn parse_return_statement(&self) -> Option<Box<dyn Statement>> {
        let return_tok = self.cur_token.borrow().clone();

        self.next_token();

        while !self.cur_tok_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(ReturnStatement::new(return_tok, None)))
    }
}