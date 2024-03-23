use crate::{
    ast::expression::{IdentifierExp, NumExp},
    ast::Expression,
    token::TokenType,
};
use crate::ast::expression::{InfixExp, PrefixExp};
use crate::ast::Statement;
use crate::token::precedence::PREFIX;

use super::Parser;

impl Parser {
    pub(super) fn register_parse_functions(&self) {
        // Register the prefix parsing functions.
        self.register_prefix(TokenType::Ident, Parser::parse_identifier);
        self.register_prefix(TokenType::IntegerNum, Parser::parse_number);
        self.register_prefix(TokenType::FloatNum, Parser::parse_number);
        self.register_prefix(TokenType::Not, Parser::parse_prefix_exp);
        self.register_prefix(TokenType::Minus, Parser::parse_prefix_exp);

        // Register the infix parsing functions.
        self.register_infix(TokenType::Plus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Minus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Star, Parser::parse_infix_exp);
        self.register_infix(TokenType::Slash, Parser::parse_infix_exp);
        self.register_infix(TokenType::EqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::NotEqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::Greater, Parser::parse_infix_exp);
        self.register_infix(TokenType::Less, Parser::parse_infix_exp);
    }

    // ==================== Prefix Parsing Functions ====================

    pub(super) fn parse_prefix_exp(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let operator = cur_token.literal().to_string();

        self.next_token();

        let right = match self.parse_expression(PREFIX) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the prefix operator.");
                return None;
            }
        };

        Some(Box::new(PrefixExp::new(cur_token, operator, right)))
    }

    // This method is used to parse the identifier expression.
    pub(super) fn parse_identifier(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let literal = cur_token.literal().to_string();
        let ident = IdentifierExp::new(cur_token, literal);

        Some(Box::new(ident))
    }

    // This method is used to parse the number expression.
    pub(super) fn parse_number(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let literal = cur_token.literal().to_string();

        // Parse the number expression by checking the token type.
        let num_exp: NumExp = match cur_token.token_type() {
            TokenType::IntegerNum => {
                let i_value = literal.parse::<i64>().unwrap();
                NumExp::new(cur_token, Some(i_value), None)
            }
            TokenType::FloatNum => {
                let f_value = literal.parse::<f64>().unwrap();
                NumExp::new(cur_token, None, Some(f_value))
            }
            _ => panic!("This is not a number token."),
        };

        Some(Box::new(num_exp))
    }

    // ==================== Infix Parsing Functions ====================

    pub(super) fn parse_infix_exp(&self, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let operator = cur_token.literal().to_string();

        let precedence = self.cur_precedence();
        self.next_token();
        let right = match self.parse_expression(precedence) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the infix operator.");
                return None;
            }
        };

        Some(Box::new(InfixExp::new(cur_token, left, operator, right)))
    }
}
