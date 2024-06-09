use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast_v2::{Expression, Node, Statement};
use crate::lexer::lexer::{Lexer, TokensIter};
use crate::token::precedence::*;
use crate::token::token::Token;
use crate::token::types::TokenType;

use super::{InfixParseFn, PrefixParseFn};

pub struct Parser {
    // The lexer that will generate tokens.
    tokens: TokensIter,

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

    // The prefix and infix parsing functions.
    prefix_parse_fns: RefCell<HashMap<TokenType, PrefixParseFn>>,
    infix_parse_fns: RefCell<HashMap<TokenType, InfixParseFn>>,
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let parser = Parser {
            tokens: Lexer::new(path).tokens_iter(),
            cur_token: RefCell::new(Rc::new(Token::new(TokenType::Illegal, "", "", 0, 0))),
            peek_token: RefCell::new(Rc::new(Token::new(TokenType::Illegal, "", "", 0, 0))),
            cmd_start_index: Cell::new(0),
            cmd_cur_index: Cell::new(0),
            programs: RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()),
            prefix_parse_fns: RefCell::new(HashMap::new()),
            infix_parse_fns: RefCell::new(HashMap::new()),
        };

        // Initialize the current and peek tokens.
        parser.next_token();
        parser.next_token();

        // Register the prefix and infix parsing functions.
        parser.register_parse_functions();

        parser.parse();

        // Check if there are any errors during parsing.
        if parser.errors.borrow().len() > 0 {
            for error in parser.errors.borrow().iter() {
                println!("{}", error);
            }
        }

        parser
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

    /// Get AST from the parser.
    pub fn programs(self) -> Vec<Node> {
        self.programs.into_inner()
    }

    // This method is used to build the AST.
    fn parse(&self) {
        while !self.cur_token.borrow().is_eof() {
            match self.parse_code() {
                Some(node) => {
                    self.programs.borrow_mut().push(node);
                    self.next_token();
                }
                None => self.next_token(),
            }
        }
    }

    pub(super) fn parse_code(&self) -> Option<Node> {
        let cur_tok = self.get_cur_token();
        match cur_tok.token_type() {
            TokenType::Let | TokenType::Return | TokenType::LeftBrace | TokenType::Func => {
                return match self.parse_stmt() {
                    Some(stmt) => Some(stmt),
                    None => None,
                };
            }
            TokenType::Ident
            | TokenType::IntegerNum
            | TokenType::FloatNum
            | TokenType::True
            | TokenType::False
            | TokenType::String
            | TokenType::Not
            | TokenType::Minus
            | TokenType::LeftParen
            | TokenType::LeftBracket
            | TokenType::If => match self.parse_expression(LEVEL_0) {
                Some(exp) => {
                    let node = Node::Exp(exp);
                    return Some(node);
                }
                None => None,
            },
            TokenType::Eof | TokenType::Semicolon => None,
            _ => {
                self.store_error(&format!(
                    "no such statement that starts with this token: {:?}",
                    cur_tok.token_type()
                ));
                None
            }
        }
    }

    pub(super) fn get_cur_token(&self) -> Rc<Token> {
        self.cur_token.borrow().clone()
    }

    fn parse_stmt(&self) -> Option<Node> {
        match self.get_cur_token().token_type() {
            TokenType::Let => match self.parse_let_stmt() {
                Some(let_stmt) => {
                    let node = Node::Stmt(Statement::Let(let_stmt));
                    Some(node)
                }
                None => None,
            },
            TokenType::Return => match self.parse_return_stmt() {
                Some(return_stmt) => {
                    let node = Node::Stmt(Statement::Return(return_stmt));
                    Some(node)
                }
                None => None,
            },
            TokenType::LeftBrace => match self.parse_block_stmt() {
                Some(block_stmt) => {
                    let node = Node::Stmt(Statement::Block(block_stmt));
                    Some(node)
                }
                None => None,
            },
            TokenType::Func => match self.parse_func_stmt() {
                Some(func_stmt) => {
                    let node = Node::Stmt(Statement::Func(func_stmt));
                    Some(node)
                }
                None => None,
            },
            _ => {
                self.store_error("There is no such statement that starts with this token.");
                None
            }
        }
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
    pub(super) fn parse_expression(&self, precedence: Precedence) -> Option<Expression> {
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
            // Move to infix operator token and parse the infix expression.
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
            let msg = format!(
                "expected next token to be `{:?}`, got `{:?}` instead",
                token_type,
                self.peek_token.borrow().token_type()
            );
            self.store_error(&msg);
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
    pub(super) fn next_token(&self) {
        // The current token is EOF means the peek token is also EOF.
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
        *self.peek_token.borrow_mut() = self.tokens.next().unwrap();
    }
}
