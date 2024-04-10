use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use crate::ast::ast::{Expression, Program, Statement};
use crate::ast::statement::ExpressionStatement;
use crate::lexer::lexer::Lexer;
use crate::parser::{InfixParseFn, PrefixParseFn};
use crate::token::precedence::{Precedence, LEVEL_0};
use crate::token::token::{Token, TokenType};

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
    program: Box<Program>,

    // Collect errors that occur during parsing.
    errors: RefCell<Vec<String>>,

    // The prefix and infix parsing functions.
    prefix_parse_fns: RefCell<HashMap<TokenType, PrefixParseFn>>,
    infix_parse_fns: RefCell<HashMap<TokenType, InfixParseFn>>,
}

impl Parser {
    pub fn new(code: &str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(code),
            cur_token: RefCell::new(Token::new(TokenType::Eof, "")),
            peek_token: RefCell::new(Token::new(TokenType::Eof, "")),
            cmd_start_index: Cell::new(0),
            cmd_cur_index: Cell::new(-1),
            program: Box::new(Program::new()),
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

    // Move the ownership of the program to the caller.
    pub fn program(self) -> Box<Program> {
        self.program
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.borrow().clone()
    }

    pub(super) fn get_cur_token(&self) -> Token {
        self.cur_token.borrow().clone()
    }

    // This method is used to register the prefix parsing functions.
    // For example: !true; -5; etc.
    pub(super) fn register_prefix(&self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.borrow_mut().insert(token_type, func);
    }

    // This method is used to register the prefix parsing functions.
    // For example: 5 + 5; 5 * 5; etc.
    pub(super) fn register_infix(&self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.borrow_mut().insert(token_type, func);
    }

    // This method is used to build the AST.
    fn parse(&mut self) {
        while self.cur_token.borrow().token_type() != &TokenType::Eof {
            match self.parse_statement() {
                Some(statement) => {
                    self.program.push(statement);
                }
                None => self.next_token(),
            }
        }
    }

    /// If the current statement is just declaration, the parser will call relevant method to parse it.
    /// For example: let x = 5; let y = 10; etc.
    /// As for expression, parser will parse it and wrap it into an [`ExpressionStatement`] by calling [`self.parse_expression_statement()`].
    pub(super) fn parse_statement(&self) -> Option<Box<dyn Statement>> {
        let cur_tok = self.cur_token.borrow().clone();
        match cur_tok.token_type() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Ident
            | TokenType::IntegerNum
            | TokenType::FloatNum
            | TokenType::Not
            | TokenType::Minus
            | TokenType::LeftParen
            | TokenType::LeftBrace
            | TokenType::True
            | TokenType::False
            | TokenType::Func
            | TokenType::If => self.parse_expression_statement(),
            TokenType::Semicolon | TokenType::Eof => None, // Semicolon means the end of a statement, and Eof means the end of the program.
            _ => {
                self.store_error("There is no such statement that starts with this token.");
                None
            }
        }
    }

    /// Parse expressions like: 5 + 5; 5 * 5; etc.
    fn parse_expression_statement(&self) -> Option<Box<dyn Statement>> {
        // The real parsing logical is [`self.parse_expression()`], calling it with the LOWEST precedence to parse the expression.
        let exp_stmt =
            ExpressionStatement::new(self.get_cur_token(), self.parse_expression(LEVEL_0));

        if self.peek_tok_is(&TokenType::Semicolon) || self.peek_tok_is(&TokenType::Eof) {
            self.next_token();
        }

        Some(Box::new(exp_stmt))
    }

    /// This method is the cornerstone of the syntax parser, and indeed, the entire Pratt parser. In parsing expressions,
    /// operator precedence is utilized for assistance.
    /// The `precedence` indicates right associativity; the higher the precedence, the stronger the right associativity.
    /// To put it more colloquially, the larger this value is, the more it can "cling" to the expression on the right
    /// and form a new expression. For example: 1 + 2 + 3,
    /// the precedence of the first '+' is higher than the numeric literal 2, hence 1 + 2 forms a new expression (1 + 2),
    /// then, the precedence of the second '+' is higher than the numeric literal 3,
    /// thus (1 + 2) combines with 3 through the second '+' to become ((1 + 2) + 3).
    /// Right associativity in the parsing process allows the token on the right to stay as close as possible to the current token,
    /// which is an alternative implementation of left recursion.
    /// The reason for using left recursion is to avoid symbol transformation that occurs with right recursion,
    /// For instance: x - y - z, using right recursion would transform it into (x - (y + z)), while using left recursion results in ((x - y) - z),
    /// This avoids semantic issues in the code after parsing is complete, even though the syntax is correct.
    pub(super) fn parse_expression(&self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        // temporary value is freed at the end of this statement,
        // so we need to store a borrow of it in a variable
        let binding = self.prefix_parse_fns.borrow();
        let prefix_func = binding.get(self.cur_token.borrow().token_type());

        // Check if the prefix parsing function exists.
        if prefix_func.is_none() {
            let msg = format!(
                "no prefix parse function for `{:?}` found",
                self.cur_token.borrow().token_type()
            );
            self.store_error(&msg);
            return None;
        }
        // Call the prefix parsing function to get corresponding AST node.
        let mut left = prefix_func.unwrap()(self);

        // Determine whether it's necessary to parse an infix expression.
        // The step `precedence < self.peek_precedence()` is crucial. For example: 5 + 5,
        // when the first '5' comes in, it's passed with the LOWEST precedence, which is the lowest priority,
        // then, `self.peek_token` becomes '+', and the next token's precedence is obtained through `peek_precedence()`,
        // which is then compared with the passed-in precedence to decide whether right associativity is needed,
        // the precedence of '+' is obviously higher than the LOWEST, therefore, this if branch is entered.
        while !self.peek_tok_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let binding = self.infix_parse_fns.borrow();
            let infix_func = binding.get(self.peek_token.borrow().token_type());

            // Call the infix parsing function to get corresponding AST node.
            if infix_func.is_none() {
                return left;
            }
            self.next_token();
            left = infix_func.unwrap()(self, left.unwrap());
        }

        left
    }

    fn peek_precedence(&self) -> u32 {
        self.peek_token.borrow().precedence()
    }

    pub(super) fn cur_precedence(&self) -> u32 {
        self.cur_token.borrow().precedence()
    }

    pub(super) fn cur_tok_is(&self, token_type: &TokenType) -> bool {
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
    pub fn expect_peek(&self, token_type: TokenType) -> bool {
        if self.peek_tok_is(&token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }

    // This method is used to parse the let statement.
    // It gets error code by calling [`lexer.joint_tokens_to_str_by_range()`] with the start and end indexes.
    pub(super) fn store_error(&self, msg: &str) {
        // Get the error code;
        let code = self
            .lexer
            .joint_tokens_to_str_by_range(self.cmd_start_index.get(), self.cmd_cur_index.get());
        // Update the start index of the next command.
        self.cmd_start_index.set(self.cmd_cur_index.get());

        // Store the error message.
        self.errors
            .borrow_mut()
            .push(format!("`{}` get error: {}", code, msg));
    }

    // Move to the next token and update the current and peek tokens.
    pub(super) fn next_token(&self) {
        *self.cur_token.borrow_mut() = self.peek_token.borrow().clone();
        *self.peek_token.borrow_mut() = self.lexer.next_token();
        self.cmd_cur_index.set(self.cmd_cur_index.get() + 1);
    }
}
