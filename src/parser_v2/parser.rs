use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::ast_v2::Statement;
use crate::lexer::lexer::TokensIter;
use crate::{
    ast_v2::Node,
    lexer::lexer::Lexer,
    token::token::{Token, TokenType},
};

pub struct Parser {
    // The lexer that will generate tokens.
    tokens_iter: TokensIter,

    // The current token that the parser is looking at.
    cur_token: RefCell<Rc<Token>>,
    // The next token that the parser will look at.
    // It is used to select which parsing function to call.
    peek_token: RefCell<Rc<Token>>,

    // Use to get command and its parameter from lexer.
    cmd_start_index: Cell<i32>,
    cmd_cur_index: Cell<i32>,

    // The root node of the AST.
    programs: RefCell<Vec<Node>>,

    // Collect errors that occur during parsing.
    errors: RefCell<Vec<String>>,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        let parser = Parser {
            tokens_iter: Lexer::new(code).tokens_iter(),
            cur_token: RefCell::new(Rc::new(Token::new(TokenType::Illegal, ""))),
            peek_token: RefCell::new(Rc::new(Token::new(TokenType::Illegal, ""))),
            cmd_start_index: Cell::new(0),
            cmd_cur_index: Cell::new(0),
            programs: RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()),
        };

        // Initialize the current and peek tokens.
        parser.next_token();
        parser.next_token();

        parser.parse();

        parser
    }

    /// Get AST from the parser.
    pub fn programs(self) -> Vec<Node> {
        self.programs.into_inner()
    }

    /// Get errors that occur during parsing.
    pub fn errors(self) -> Vec<String> {
        self.errors.into_inner()
    }

    // This method is used to build the AST.
    fn parse(&self) {
        while !self.cur_token.borrow().is_eof() {
            match self.parse_code() {
                Some(statement) => {
                    self.programs.borrow_mut().push(statement);
                }
                None => self.next_token(),
            }
        }
    }

    fn parse_code(&self) -> Option<Node> {
        let cur_tok = self.get_cur_token();
        match cur_tok.token_type() {
            TokenType::Let => match self.parse_let_stmt() {
                Some(let_stmt) => {
                    let node = Node::Stmt(Statement::Let(let_stmt));
                    return Some(node);
                }
                None => return None,
            },
            TokenType::Return => {
                self.parse_return_stmt();
                return None;
            }
            _ => None,
        }
    }

    pub(super) fn get_cur_token(&self) -> Rc<Token> {
        self.cur_token.borrow().clone()
    }

    pub(super) fn cur_token_is(&self, token_type: &TokenType) -> bool {
        self.cur_token.borrow().token_type() == token_type
    }

    pub(super) fn peek_tok_is(&self, token_type: &TokenType) -> bool {
        if self.peek_token.borrow().token_type() == token_type {
            true
        } else {
            let msg = format!(
                "expected next token to be `{:?}`, got `{:?}` instead",
                token_type,
                self.peek_token.borrow().token_type()
            );
            self.store_error(&msg);
            false
        }
    }

    /// This method is used to check if the next token is of the expected type.
    /// It's mainly used to ensure the sequence of the tokens is correct.
    pub fn expect_peek(&self, token_type: &TokenType) -> bool {
        if self.peek_tok_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }

    // This method is used to parse the let statement.
    // It gets error code by calling [`lexer.joint_tokens_to_str_by_range()`] with the start and end indexes.
    pub(super) fn store_error(&self, msg: &str) {
        // Update the start index of the next command.
        self.cmd_start_index.set(self.cmd_cur_index.get());

        // Store the error message.
        self.errors.borrow_mut().push(format!("get error: {}", msg));
    }

    /// Update the [`cur_token`] and [`peek_token`].
    pub(super) fn next_token(&self) {// The current token is EOF means the peek token is also EOF.
        // So is no need to update the tokens.
        if (*self.cur_token.borrow()).is_eof() {
            return;
        }
        *self.cur_token.borrow_mut() = self.peek_token.borrow().clone();

        // If the peek token already is EOF, it means the current token is the last token,
        // and haven't more tokens to read.
        if (*self.peek_token.borrow()).is_eof() {
            return;
        }
        *self.peek_token.borrow_mut() = self.tokens_iter.next().unwrap();
    }
}
