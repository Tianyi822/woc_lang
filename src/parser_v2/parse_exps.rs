use crate::{
    ast_v2::{
        expressions::{
            BooleanExp, CallExp, ElseExp, IdentifierExp, IfExp, InfixExp, NumExp, PrefixExp,
        },
        statements::BlockStatement,
        Expression,
    },
    token::{precedence::*, token::TokenType},
};

use super::parser::Parser;

impl Parser {
    pub(super) fn register_parse_functions(&self) {
        // Register the prefix parsing functions.
        self.register_prefix(TokenType::Not, Parser::parse_prefix_exp);
        self.register_prefix(TokenType::Minus, Parser::parse_prefix_exp);
        self.register_prefix(TokenType::Ident, Parser::parse_identifier);
        self.register_prefix(TokenType::IntegerNum, Parser::parse_number);
        self.register_prefix(TokenType::FloatNum, Parser::parse_number);
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
        self.register_infix(TokenType::GreaterThanOrEqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::Less, Parser::parse_infix_exp);
        self.register_infix(TokenType::LessThanOrEqualTo, Parser::parse_infix_exp);
        self.register_infix(TokenType::And, Parser::parse_infix_exp);
        self.register_infix(TokenType::Or, Parser::parse_infix_exp);
        self.register_infix(TokenType::LeftParen, Parser::parse_call_exp);
    }

    // ==================== Prefix Parsing Functions ====================

    fn parse_prefix_exp(&self) -> Option<Expression> {
        let cur_token = self.get_cur_token();

        self.next_token();

        let right = match self.parse_expression(PREFIX) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the prefix operator.");
                return None;
            }
        };

        Some(Expression::Prefix(PrefixExp::new(
            cur_token.token_type().clone(),
            right,
        )))
    }

    // This method is used to parse the identifier expression.
    pub(super) fn parse_identifier(&self) -> Option<Expression> {
        let literal = self.get_cur_token().literal().to_string();
        let ident = IdentifierExp::new(literal);

        Some(Expression::Identifier(ident))
    }

    // This method is used to parse the number expression.
    fn parse_number(&self) -> Option<Expression> {
        let cur_token = self.get_cur_token();
        let literal = cur_token.literal().to_string();

        // Parse the number expression by checking the token type.
        let num_exp: NumExp = match cur_token.token_type() {
            TokenType::IntegerNum => {
                let i_value = literal.parse::<i64>().unwrap();
                NumExp::new(Some(i_value), None)
            }
            TokenType::FloatNum => {
                let f_value = literal.parse::<f64>().unwrap();
                NumExp::new(None, Some(f_value))
            }
            _ => panic!("This is not a number token."),
        };

        Some(Expression::Num(num_exp))
    }

    // This method is used to parse the grouped expression: (5 + 5).
    fn parse_grouped_exp(&self) -> Option<Expression> {
        self.next_token();

        let exp = self.parse_expression(LEVEL_0);

        if !self.expect_peek(&TokenType::RightParen) {
            return None;
        }

        exp
    }

    // This method is used to parse the boolean expression.
    fn parse_boolean(&self) -> Option<Expression> {
        let cur_token = self.get_cur_token();
        let value = match cur_token.token_type() {
            TokenType::True => true,
            TokenType::False => false,
            _ => panic!("This is not a boolean token."),
        };

        Some(Expression::Boolean(BooleanExp::new(value)))
    }

    fn parse_if_expression(&self) -> Option<Expression> {
        if !self.expect_peek(&TokenType::LeftParen) {
            self.store_error("There is no left parenthesis after the if keyword.");
            return None;
        }

        // Get the condition expressions
        self.next_token();
        let condition = match self.parse_expression(LEVEL_0) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the if keyword.");
                return None;
            }
        };

        if !self.expect_peek(&TokenType::RightParen) {
            self.store_error("There is no right parenthesis after the condition expression.");
            return None;
        }

        if !self.expect_peek(&TokenType::LeftBrace) {
            self.store_error("There is no left brace after the right parenthesis.");
            return None;
        }

        // Get the consequence block statement
        let consequence = match self.parse_block_stmt() {
            Some(block) => block,
            None => {
                self.store_error("There is no block statement after the left brace.");
                return None;
            }
        };

        // Get the else expression if it exists
        let else_exp = if self.peek_tok_is(&TokenType::Else) {
            // Skip the else token
            self.next_token();

            // Move to the next token
            self.next_token();

            let mut if_exp: Option<Expression> = None;
            let mut consequence: Option<BlockStatement> = None;

            if self.cur_tok_is(&TokenType::If) {
                if_exp = self.parse_if_expression();
            } else if self.cur_tok_is(&TokenType::LeftBrace) {
                consequence = self.parse_block_stmt();
            } else {
                self.store_error("There is no else expression after the else keyword.");
                return None;
            }

            if if_exp.is_some() {
                Some(ElseExp::new(Some(Box::new(if_exp.unwrap())), consequence))
            } else {
                Some(ElseExp::new(None, consequence))
            }
        } else {
            None
        };

        Some(Expression::If(IfExp::new(condition, consequence, else_exp)))
    }

    // ==================== Infix Parsing Functions ====================

    /// This method is used to parse the infix expression.
    fn parse_infix_exp(&self, left: Expression) -> Option<Expression> {
        let cur_token = self.get_cur_token();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = match self.parse_expression(precedence) {
            Some(exp) => exp,
            None => {
                self.store_error("There is no expression after the infix operator.");
                return None;
            }
        };

        Some(Expression::Infix(InfixExp::new(
            left,
            cur_token.token_type().clone(),
            right,
        )))
    }

    /// This method is used to parse the call expression.
    /// A call expression is an expression that calls a function.
    /// For example:
    ///
    /// ```
    /// add(5, 5);
    /// ```
    fn parse_call_exp(&self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments();

        Some(Expression::Call(CallExp::new(function, arguments)))
    }

    /// This method is used to parse the call arguments.
    /// For example:
    ///
    /// ```
    /// add(5, 5);
    /// ```
    fn parse_call_arguments(&self) -> Vec<Expression> {
        let mut args = Vec::new();

        if self.peek_tok_is(&TokenType::RightParen) {
            self.next_token();
            return args;
        }

        self.next_token();
        args.push(self.parse_expression(LEVEL_0).unwrap());

        while self.peek_tok_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(LEVEL_0).unwrap());
        }

        if !self.expect_peek(&TokenType::RightParen) {
            return Vec::new();
        }

        args
    }
}
