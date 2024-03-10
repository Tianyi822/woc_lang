use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Parser {
    // The lexer that will generate tokens.
    lexer: Lexer,

    // The current token that the parser is looking at.
    cur_token: RefCell<Token>,

    // Use to get command and its parameter from lexer.
    cmd_start_index: Cell<u32>,
    cmd_end_index: Cell<u32>,

    // Collect errors that occur during parsing.
    errors: RefCell<Vec<String>>,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        let parser = Self {
            lexer: Lexer::new(code),
            cur_token: RefCell::new(Token::new(TokenType::Eof, "")),
            cmd_start_index: Cell::new(0),
            cmd_end_index: Cell::new(0),
            errors: RefCell::new(Vec::new()),
        };

        // Initialize the current token.
        parser.next_token();
        // Clear the lexer.
        parser.lexer.clear();

        parser
    }

    fn parse(&self) {
        // Start parsing and build the AST.
        loop {
            if *self.cur_token.borrow().token_type() == TokenType::Eof {
                return;
            }
        }
    }

    fn next_token(&self) {
        let mut cur_token = self.cur_token.borrow_mut();

        match self.lexer.next_token() {
            Some(t) => *cur_token = t,
            None => *cur_token = Token::new(TokenType::Eof, ""),
        }

        self.cmd_end_index.set(self.cmd_end_index.get() + 1);
    }
}