use crate::{
    ast_v2::{
        expressions::IdentifierExp,
        statements::{BlockStatement, FuncStatement, LetStatement, ReturnStatement},
    },
};
use crate::token::precedence::LEVEL_0;
use crate::token::types::TokenType;

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

    /// Parse the block statement.
    /// A block statement is a series of statements enclosed in braces.
    /// For example:
    ///
    /// ```
    /// {
    ///    let x = 5;
    ///   let y = 10;
    /// }
    /// ```
    pub(super) fn parse_block_stmt(&self) -> Option<BlockStatement> {
        let mut block = BlockStatement::new();

        // Move to the next token
        self.next_token();

        while !self.cur_tok_is(&TokenType::RightBrace) && !self.cur_tok_is(&TokenType::Eof) {
            if let Some(stmt) = self.parse_code() {
                block.add(Box::new(stmt));
            }

            self.next_token();
        }

        Some(block)
    }

    /// Parse the function statement.
    /// A function statement is a statement that defines a function.
    /// For example:
    ///
    /// ```
    /// func add(x, y) { return x + y; }
    /// ```
    pub(super) fn parse_func_stmt(&self) -> Option<FuncStatement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        // Move to Ident token
        let name = self.get_cur_token().literal().to_string();
        // Create IdentifierExp
        let ident_exp = IdentifierExp::new(name);

        // Move to LeftParen token
        if !self.expect_peek(&TokenType::LeftParen) {
            return None;
        }

        // Parse the function parameters
        let params = self.parse_func_parameters();

        // Move to LeftBrace token
        if !self.expect_peek(&TokenType::LeftBrace) {
            return None;
        }

        // Parse the block statement
        let block = match self.parse_block_stmt() {
            Some(block) => block,
            None => {
                self.store_error("There is no block statement in the function statement.");
                return None;
            }
        };

        Some(FuncStatement::new(ident_exp, params, block))
    }

    /// Parse the function parameters.
    /// For example:
    ///
    /// ```
    /// func add(x, y) { return x + y; }
    /// ```
    fn parse_func_parameters(&self) -> Option<Vec<IdentifierExp>> {
        let mut params = Vec::new();

        // Check if there are no parameters
        if self.peek_tok_is(&TokenType::RightParen) {
            // Move to RightParen token
            self.next_token();
            return None;
        }

        // Move to the next token
        self.next_token();
        let literal = self.get_cur_token().literal().to_string();
        let ident = IdentifierExp::new(literal);
        params.push(ident);

        while self.peek_tok_is(&TokenType::Comma) {
            // Skip Comma token
            self.next_token();

            // Move to next parameter
            self.next_token();

            let literal = self.get_cur_token().literal().to_string();
            let ident = IdentifierExp::new(literal);
            params.push(ident);
        }

        if !self.expect_peek(&TokenType::RightParen) {
            return None;
        }

        Some(params)
    }
}
