use std::cell::{Cell, RefCell};

use crate::ast::expression::IdentifierExp;
use crate::ast::statement::LetState;
use crate::ast::{Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub mod parse_exp;
pub mod parse_stmt;

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
        };

        // Initialize the current and peek tokens.
        parser.next_token();
        parser.next_token();
        // Start parsing the program.
        parser.parse();
        // Clear the lexer.
        parser.lexer.clear();

        parser
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
            TokenType::Eof | _ => {
                self.store_error("There is no such statement that starts with this token.");
                None
            }
        }
    }

    // This method is used to parse the let statement.
    fn parse_let_statement(&self) -> Option<Box<dyn Statement>> {
        let let_tok = self.cur_token.borrow().clone();

        if !self.expect_peek(TokenType::Literal) {
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

        Some(Box::new(LetState::new(let_tok, ident, None)))
    }

    fn cur_tok_is(&self, token_type: TokenType) -> bool {
        self.cur_token.borrow().token_type() == &token_type
    }

    // This method is used to check if the next token is of the expected type.
    // It's mainly used to ensure the sequence of the tokens is correct.
    fn expect_peek(&self, token_type: TokenType) -> bool {
        if self.peek_token.borrow().token_type() == &token_type {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
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

    fn peek_error(&self, token_type: TokenType) {
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
