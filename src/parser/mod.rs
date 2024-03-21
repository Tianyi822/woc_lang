use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::ast::statement::ExpressionStatement;
use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::precedence::*;
use crate::token::{Token, TokenType};

pub mod parse_exp_functions;
pub mod parse_stmt;

type PrefixParseFn = fn(&Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&Parser, Box<dyn Statement>) -> Option<Box<dyn Expression>>;

pub struct Parser {
    // The lexer that will generate tokens.
    lexer: Lexer,

    // The current token that the parser is looking at.
    cur_token: RefCell<Token>,
    // The next token that the parser will look at.
    // It is used to select which parsing function to call.
    peek_token: RefCell<Token>,

    // Use to get command and its parameter from lexer.
    cmd_start_index: Cell<i32>,
    cmd_cur_index: Cell<i32>,

    // The root node of the AST.
    pub program: Program,

    // Collect errors that occur during parsing.
    pub errors: RefCell<Vec<String>>,

    // The prefix and infix parsing functions.
    prefix_parse_fns: RefCell<HashMap<TokenType, PrefixParseFn>>,
    infix_parse_fns: RefCell<HashMap<TokenType, InfixParseFn>>,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        let parser = Self {
            lexer: Lexer::new(code),
            cur_token: RefCell::new(Token::new(TokenType::Eof, "")),
            peek_token: RefCell::new(Token::new(TokenType::Eof, "")),
            cmd_start_index: Cell::new(0),
            cmd_cur_index: Cell::new(-1),
            program: Program::new(),
            errors: RefCell::new(Vec::new()),
            prefix_parse_fns: RefCell::new(HashMap::new()),
            infix_parse_fns: RefCell::new(HashMap::new()),
        };

        // Initialize the current and peek tokens.
        parser.next_token();
        parser.next_token();

        // Register the prefix and infix parsing functions.
        parser.register_parse_functions();

        // Start parsing the program.
        parser.parse();
        // Clear the lexer.
        parser.lexer.clear();

        parser
    }

    fn cur_token(&self) -> Token {
        self.cur_token.borrow().clone()
    }

    // This method is used to register the prefix parsing functions.
    // For example: !true; -5; etc.
    fn register_prefix(&self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.borrow_mut().insert(token_type, func);
    }

    // This method is used to register the prefix parsing functions.
    // For example: 5 + 5; 5 * 5; etc.
    fn register_infix(&self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.borrow_mut().insert(token_type, func);
    }

    // This method is used to build the AST.
    fn parse(&self) {
        while self.cur_token.borrow().token_type() != &TokenType::Eof {
            match self.parse_statement() {
                Some(statement) => {
                    self.program.push(statement);
                }
                None => self.next_token(),
            }
        }
    }

    fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        let cur_tok = self.cur_token.borrow().clone();
        match cur_tok.token_type() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Ident
            | TokenType::IntegerNum
            | TokenType::FloatNum
            | TokenType::True
            | TokenType::False
            | TokenType::Func => self.parse_expression_statement(),
            _ => {
                self.store_error("There is no such statement that starts with this token.");
                None
            }
        }
    }

    // Parse expressions like: 5 + 5; 5 * 5; etc.
    fn parse_expression_statement(&self) -> Option<Box<dyn Statement>> {
        let exp_stmt = ExpressionStatement::new(
            self.cur_token(),
            self.parse_expression(LEVEL_0),
        );

        if self.peek_tok_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(Box::new(exp_stmt))
    }

    fn parse_expression(&self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        // temporary value is freed at the end of this statement,
        // so we need to store a borrow of it in a variable
        let binding = self.prefix_parse_fns.borrow();
        let prefix_func = binding.get(self.cur_token.borrow().token_type());

        if prefix_func.is_none() {
            let msg = format!(
                "no prefix parse function for `{:?}` found",
                self.cur_token.borrow().token_type()
            );
            self.store_error(&msg);
            return None;
        }

        prefix_func.unwrap()(self)
    }

    fn cur_tok_is(&self, token_type: &TokenType) -> bool {
        self.cur_token.borrow().token_type() == token_type
    }

    fn peek_tok_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.borrow().token_type() == token_type
    }

    // This method is used to check if the next token is of the expected type.
    // It's mainly used to ensure the sequence of the tokens is correct.
    fn expect_peek(&self, token_type: TokenType) -> bool {
        if self.peek_tok_is(&token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&token_type);
            false
        }
    }

    fn store_error(&self, msg: &str) {
        // Get the error code;
        let code = self
            .lexer
            .joint_tokens_to_str_by_range(self.cmd_start_index.get(), self.cmd_cur_index.get());
        // Update the start index of the next command.
        self.cmd_start_index.set(self.cmd_cur_index.get());

        let error = format!("`{}` get error: {}", code, msg);
        self.errors.borrow_mut().push(error);
    }

    fn peek_error(&self, token_type: &TokenType) {
        let msg = format!(
            "expected next token to be `{:?}`, got `{:?}` instead",
            token_type,
            self.peek_token.borrow().token_type()
        );
        self.store_error(&msg);
    }

    fn next_token(&self) {
        *self.cur_token.borrow_mut() = self.peek_token.borrow().clone();
        *self.peek_token.borrow_mut() = self.lexer.next_token();
        self.cmd_cur_index.set(self.cmd_cur_index.get() + 1);
    }
}
