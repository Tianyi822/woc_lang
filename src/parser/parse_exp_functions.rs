use super::Parser;
use crate::ast::ast::Expression;
use crate::ast::expression::{BooleanExp, IdentifierExp, InfixExp, NumExp, PrefixExp};
use crate::token::precedence::{LEVEL_0, PREFIX};
use crate::token::token::TokenType;

impl Parser {
    pub(super) fn register_parse_functions(&self) {
        // Register the prefix parsing functions.
        self.register_prefix(TokenType::Ident, Parser::parse_identifier);
        self.register_prefix(TokenType::IntegerNum, Parser::parse_number);
        self.register_prefix(TokenType::FloatNum, Parser::parse_number);
        self.register_prefix(TokenType::Not, Parser::parse_prefix_exp);
        self.register_prefix(TokenType::Minus, Parser::parse_prefix_exp);
        self.register_prefix(TokenType::LeftParen, Parser::parse_grouped_exp);
        self.register_prefix(TokenType::True, Parser::parse_boolean);
        self.register_prefix(TokenType::False, Parser::parse_boolean);
        self.register_prefix(TokenType::If, Parser::parse_if_expression);

        // Register the infix parsing functions.
        self.register_infix(TokenType::Plus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Minus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Asterisk, Parser::parse_infix_exp);
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

    // This method is used to parse the boolean expression.
    pub(super) fn parse_boolean(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let value = match cur_token.token_type() {
            TokenType::True => true,
            TokenType::False => false,
            _ => panic!("This is not a boolean token."),
        };

        Some(Box::new(BooleanExp::new(cur_token, value)))
    }

    // This method is used to parse the grouped expression: (5 + 5).
    pub(super) fn parse_grouped_exp(&self) -> Option<Box<dyn Expression>> {
        self.next_token();

        let exp = self.parse_expression(LEVEL_0);

        if !self.expect_peek(TokenType::RightParen) {
            return None;
        }

        exp
    }

    // This method is used to parse if expression: if (5 < 10) { return 5; } else { return 10; }
    pub(super) fn parse_if_expression(&self) -> Option<Box<dyn Expression>> {
        todo!("Implement if expression")
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
