use crate::ast::ast::Expression;
use crate::ast::expression::{
    BooleanExp, CallExp, FunctionExp, IdentifierExp, IfExp, InfixExp, NumExp, PrefixExp,
};
use crate::ast::statement::BlockStatement;
use crate::token::precedence::{LEVEL_0, PREFIX};
use crate::token::token::TokenType;

use super::Parser;

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
        self.register_prefix(TokenType::Func, Parser::parse_func_exp);

        // Register the infix parsing functions.
        self.register_infix(TokenType::Plus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Minus, Parser::parse_infix_exp);
        self.register_infix(TokenType::Asterisk, Parser::parse_infix_exp);
        self.register_infix(TokenType::Slash, Parser::parse_infix_exp);
        self.register_infix(TokenType::EqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::NotEqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::Greater, Parser::parse_infix_exp);
        self.register_infix(TokenType::Less, Parser::parse_infix_exp);
        self.register_infix(TokenType::LeftParen, Parser::parse_call_exp);
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
        let cur_tok = self.get_cur_token();

        if !self.expect_peek(TokenType::LeftParen) {
            self.store_error("There is no left parenthesis after the if keyword.");
            return None;
        }

        self.next_token();
        let condition = match self.parse_expression(LEVEL_0) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the if keyword.");
                return None;
            }
        };

        if !self.expect_peek(TokenType::RightParen) {
            self.store_error("There is no right parenthesis after the condition expression.");
            return None;
        }

        if !self.expect_peek(TokenType::LeftBrace) {
            self.store_error("There is no left brace after the right parenthesis.");
            return None;
        }

        let consequence = match self.parse_block_statement() {
            Some(block) => block,
            None => {
                self.store_error("There is no block statement after the left brace.");
                return None;
            }
        };

        let mut alternative: Option<BlockStatement> = None;

        if self.expect_peek(TokenType::Else) {
            if !self.expect_peek(TokenType::LeftBrace) {
                self.store_error("There is no left brace after the else keyword.");
                return None;
            }

            alternative = match self.parse_block_statement() {
                Some(block) => Some(block),
                None => {
                    self.store_error("There is no block statement after the left brace.");
                    return None;
                }
            };
        }

        Some(Box::new(IfExp::new(
            cur_tok,
            condition,
            consequence,
            alternative,
        )))
    }

    // This method is used to parse the function expression.
    pub(super) fn parse_func_exp(&self) -> Option<Box<dyn Expression>> {
        let cur_tok = self.get_cur_token();

        // Get function name
        let func_name = match self.expect_peek(TokenType::Ident) {
            true => {
                let name_tok = self.get_cur_token();
                let name = name_tok.literal().to_string();
                IdentifierExp::new(name_tok, name)
            }
            false => {
                self.store_error("There is no function name after the function keyword.");
                return None;
            }
        };

        // Parse the function parameters
        if !self.peek_tok_is(&TokenType::LeftParen) {
            self.store_error("There is no left parenthesis after the function keyword.");
            return None;
        }
        let parameters = self.parse_function_parameters();

        // Parse the function body
        if !self.expect_peek(TokenType::LeftBrace) {
            self.store_error("There is no left brace after the function parameters.");
            return None;
        }
        let body = match self.parse_block_statement() {
            Some(block) => block,
            None => {
                self.store_error("There is no block statement after the left brace.");
                return None;
            }
        };

        Some(Box::new(FunctionExp::new(
            cur_tok, func_name, parameters, body,
        )))
    }

    // Parsing the function parameters: func add(x, y)
    fn parse_function_parameters(&self) -> Option<Vec<IdentifierExp>> {
        if self.expect_peek(TokenType::LeftParen) {
            self.next_token();
        } else {
            return None;
        }

        let mut identifiers: Vec<IdentifierExp> = Vec::new();

        let ident = IdentifierExp::new(
            self.get_cur_token(),
            self.get_cur_token().literal().to_string(),
        );
        identifiers.push(ident);

        while self.peek_tok_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();

            let ident = IdentifierExp::new(
                self.get_cur_token(),
                self.get_cur_token().literal().to_string(),
            );
            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::RightParen) {
            self.store_error("There is no right parenthesis after the function parameters.");
            return None;
        }

        Some(identifiers)
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

    pub(super) fn parse_call_exp(
        &self,
        function: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let cur_token = self.get_cur_token();
        let arguments = self.parse_call_arguments();

        Some(Box::new(CallExp::new(cur_token, function, arguments)))
    }

    fn parse_call_arguments(&self) -> Option<Vec<Box<dyn Expression>>> {
        if !self.cur_tok_is(&TokenType::LeftParen) {
            self.store_error("There is no left parenthesis after the function name.");
            return None;
        }

        let mut args: Vec<Box<dyn Expression>> = Vec::new();

        if self.peek_tok_is(&TokenType::RightParen) {
            self.next_token();
            return Some(args);
        }

        // Move to the first argument and store it
        self.next_token();
        let arg = match self.parse_expression(LEVEL_0) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the left parenthesis.");
                return None;
            }
        };
        args.push(arg);

        // Store others arguments
        while self.peek_tok_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();

            let arg = match self.parse_expression(LEVEL_0) {
                Some(exp) => exp,
                None => {
                    self.store_error("There is no expression after the comma.");
                    return None;
                }
            };
            args.push(arg);
        }

        if !self.expect_peek(TokenType::RightParen) {
            self.store_error("There is no right parenthesis after the arguments.");
            return None;
        }

        Some(args)
    }
}
