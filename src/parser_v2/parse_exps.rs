use crate::{
    ast_v2::{
        Expression,
        expressions::{IdentifierExp, NumExp, PrefixExp},
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

        // Register the infix parsing functions.
    }

    // ==================== Prefix Parsing Functions ====================

    fn parse_prefix_exp(&self) -> Option<Expression> {
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

        Some(Expression::Prefix(PrefixExp::new(operator, right)))
    }

    // This method is used to parse the identifier expression.
    fn parse_identifier(&self) -> Option<Expression> {
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

    // ==================== Infix Parsing Functions ====================
}
