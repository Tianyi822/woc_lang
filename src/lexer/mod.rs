use std::cell::{Cell, RefCell};

use crate::token::{Token, TokenType};

use self::state::State;

mod state;

#[derive(Debug)]
pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // Start index of token in command.
    start_index: RefCell<usize>,

    // Current index of token vector.
    cur_index: RefCell<usize>,

    // Store the tokens that are parsed.
    tokens: RefCell<Vec<Token>>,

    // This is a key field to show the state about lexer at now.
    // It's used to define the type of the token currently.
    cur_state: RefCell<State>,

    // Current index of token vector.
    // This field is used to iterate the tokens.
    position: Cell<usize>,
}

impl Lexer {
    /// Creates a new [`Lexer`].
    pub fn new(command: &str) -> Lexer {
        let l = Lexer {
            command: command.chars().collect(),
            start_index: RefCell::new(0),
            cur_index: RefCell::new(0),
            tokens: RefCell::new(Vec::new()),
            cur_state: RefCell::new(State::StartState),
            position: Cell::new(0),
        };

        l.analyze_command();

        l
    }

    // Get the tokens by range.
    pub fn joint_tokens_to_str_by_range(&self, start: i32, end: i32) -> String {
        let tokens = self.tokens.borrow();
        let mut result = String::new();

        // Iterate the tokens and get the literal of token.
        for i in start..end {
            // If the token is EOF, we need to break the loop, it means the end of the code.
            if tokens[i as usize].token_type() == &TokenType::Eof {
                break;
            }

            result.push_str(tokens[i as usize].literal());
            if i < end - 1 && tokens[(i + 1) as usize].token_type() != &TokenType::Eof {
                result.push(' ');
            }
        }

        result
    }

    // Clear the lexer data.
    pub fn clear(&self) {
        self.tokens.borrow_mut().clear();
    }

    // Iterate the tokens.
    pub fn next_token(&self) -> Token {
        let tokens = self.tokens.borrow();
        let position = self.position.get();

        if position >= tokens.len() {
            return tokens[position - 1].clone();
        }

        let token = tokens[position].clone();
        self.position.set(position + 1);

        token
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&self) {
        // Iterate the command char by char.
        for (index, c) in self.command.iter().enumerate() {
            // Update self.cur_index.
            *self.cur_index.borrow_mut() = index;

            // If the char is blank, we need to store the token and transform the state.
            if c.is_whitespace() {
                self.store_token_and_trans_state();
                continue;
            }

            let state = self.cur_state.borrow().clone();
            match state {
                State::StartState => self.trans_state(c),

                // =============== keywords ===============
                // ============ while ============
                State::WhileState1 => {
                    if c.eq(&'h') {
                        self.set_state(State::WhileState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::WhileState2 => {
                    if c.eq(&'i') {
                        self.set_state(State::WhileState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::WhileState3 => {
                    if c.eq(&'l') {
                        self.set_state(State::WhileState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::WhileState4 => {
                    if c.eq(&'e') {
                        self.set_state(State::WhileState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ for ============
                State::ForState1 => {
                    if c.eq(&'o') {
                        self.set_state(State::ForState2)
                    } else if c.eq(&'u') {
                        self.set_state(State::FuncState2);
                    } else if c.eq(&'a') {
                        self.set_state(State::FalseState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ForState2 => {
                    if c.eq(&'r') {
                        self.set_state(State::ForState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ if ============
                State::IfState1 => {
                    if c.eq(&'f') {
                        self.set_state(State::IfState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ else ============
                State::ElseState1 => {
                    if c.eq(&'l') {
                        self.set_state(State::ElseState2);
                    } else if c.eq(&'n') {
                        self.set_state(State::EnumState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ElseState2 => {
                    if c.eq(&'s') {
                        self.set_state(State::ElseState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ElseState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::ElseState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ break ============
                State::BreakState1 => {
                    if c.eq(&'r') {
                        self.set_state(State::BreakState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::BreakState2 => {
                    if c.eq(&'e') {
                        self.set_state(State::BreakState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::BreakState3 => {
                    if c.eq(&'a') {
                        self.set_state(State::BreakState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::BreakState4 => {
                    if c.eq(&'k') {
                        self.set_state(State::BreakState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ continue ============
                State::ContinueState1 => {
                    if c.eq(&'o') {
                        self.set_state(State::ContinueState2)
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::ContinueState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState3 => {
                    if c.eq(&'t') {
                        self.set_state(State::ContinueState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState4 => {
                    if c.eq(&'i') {
                        self.set_state(State::ContinueState5);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState5 => {
                    if c.eq(&'n') {
                        self.set_state(State::ContinueState6);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState6 => {
                    if c.eq(&'u') {
                        self.set_state(State::ContinueState7);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ContinueState7 => {
                    if c.eq(&'e') {
                        self.set_state(State::ContinueState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ let ============
                State::LetState1 => {
                    if c.eq(&'e') {
                        self.set_state(State::LetState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::LetState2 => {
                    if c.eq(&'t') {
                        self.set_state(State::LetState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ function ============
                State::FuncState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::FuncState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::FuncState3 => {
                    if c.eq(&'c') {
                        self.set_state(State::FuncState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ return ============
                State::ReturnState1 => {
                    if c.eq(&'e') {
                        self.set_state(State::ReturnState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ReturnState2 => {
                    if c.eq(&'t') {
                        self.set_state(State::ReturnState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ReturnState3 => {
                    if c.eq(&'u') {
                        self.set_state(State::ReturnState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ReturnState4 => {
                    if c.eq(&'r') {
                        self.set_state(State::ReturnState5);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::ReturnState5 => {
                    if c.eq(&'n') {
                        self.set_state(State::ReturnState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ struct ============
                State::StructState1 => {
                    if c.eq(&'t') {
                        self.set_state(State::StructState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::StructState2 => {
                    if c.eq(&'r') {
                        self.set_state(State::StructState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::StructState3 => {
                    if c.eq(&'u') {
                        self.set_state(State::StructState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::StructState4 => {
                    if c.eq(&'c') {
                        self.set_state(State::StructState5);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::StructState5 => {
                    if c.eq(&'t') {
                        self.set_state(State::StructState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // ============ enum ============
                State::EnumState2 => {
                    if c.eq(&'u') {
                        self.set_state(State::EnumState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::EnumState3 => {
                    if c.eq(&'m') {
                        self.set_state(State::EnumState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // =============== none ===============
                State::NoneState1 => {
                    if c.eq(&'o') {
                        self.set_state(State::NoneState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::NoneState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::NoneState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::NoneState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::NoneState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // =============== true ===============
                State::TrueState1 => {
                    if c.eq(&'r') {
                        self.set_state(State::TrueState2);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::TrueState2 => {
                    if c.eq(&'u') {
                        self.set_state(State::TrueState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::TrueState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::TrueState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                // =============== false ===============
                State::FalseState2 => {
                    if c.eq(&'l') {
                        self.set_state(State::FalseState3);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::FalseState3 => {
                    if c.eq(&'s') {
                        self.set_state(State::FalseState4);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::FalseState4 => {
                    if c.eq(&'e') {
                        self.set_state(State::FalseState);
                    } else {
                        self.set_state(State::IdentState);
                    }
                }

                State::WhileState
                | State::ForState
                | State::IfState
                | State::ElseState
                | State::BreakState
                | State::ContinueState
                | State::LetState
                | State::FuncState
                | State::ReturnState
                | State::StructState
                | State::EnumState
                | State::NoneState
                | State::TrueState
                | State::FalseState => self.store_token_and_trans_state(),

                // =============== number ===============
                State::NumState => {
                    if c.is_numeric() || (state == State::NumState && c.eq(&'_')) {
                        self.set_state(State::NumState)
                    } else if c.eq(&'.') {
                        self.set_state(State::FloatNumState)
                    } else if c.is_alphabetic() {
                        self.set_state(State::IdentState)
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::FloatNumState => {
                    if c.is_numeric() {
                        self.set_state(State::FloatNumState)
                    } else if c.is_alphabetic() {
                        self.set_state(State::IdentState)
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                // =============== Literal ===============
                State::IdentState => {
                    if !(c.is_alphanumeric() || c.eq(&'_')) {
                        self.store_token_and_trans_state();
                    }
                }

                // =============== single symbols ===============
                State::CommaState
                | State::DotState
                | State::SemiColonState
                | State::ColonState
                | State::LeftParenState
                | State::RightParenState
                | State::LeftBraceState
                | State::RightBraceState
                | State::LeftBracketState
                | State::RightBracketState
                | State::BitNotState
                | State::PercentState
                | State::QuoteState
                | State::SingleQuoteState => {
                    self.store_token_and_trans_state();
                }

                // =============== combined symbols ===============
                State::AssignmentState => {
                    if c.eq(&'=') {
                        self.set_state(State::EqualToState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::GreaterState => {
                    if c.eq(&'=') {
                        self.set_state(State::GreaterThanOrEqualToState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::LessState => {
                    if c.eq(&'=') {
                        self.set_state(State::LessThanOrEqualToState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::NotState => {
                    if c.eq(&'=') {
                        self.set_state(State::NotEqualToState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::BitAndState => {
                    if c.eq(&'&') {
                        self.set_state(State::AndState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::BitOrState => {
                    if c.eq(&'|') {
                        self.set_state(State::OrState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::PlusState => {
                    if c.eq(&'=') {
                        self.set_state(State::PlusAssignState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::MinusState => {
                    if c.eq(&'=') {
                        self.set_state(State::MinusAssignState);
                    } else if c.is_numeric() {
                        self.set_state(State::NumState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::StarState => {
                    if c.eq(&'=') {
                        self.set_state(State::StarAssignState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::SlashState => {
                    if c.eq(&'=') {
                        self.set_state(State::SlashAssignState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::AndState
                | State::OrState
                | State::EqualToState
                | State::NotEqualToState
                | State::GreaterThanOrEqualToState
                | State::LessThanOrEqualToState
                | State::PlusAssignState
                | State::MinusAssignState
                | State::StarAssignState
                | State::SlashAssignState => {
                    self.store_token_and_trans_state();
                }

                // ============ Underline ============
                State::UnderscoreState => {
                    if c.is_alphabetic() || c.is_numeric() {
                        self.set_state(State::IdentState);
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                // =============== end ===============
                State::EndState => break,
            }
        }

        // If the lexer's state is not end, we need to store the last token.
        if *self.cur_state.borrow() != State::EndState {
            // When the last token was completed, we need to store the token and transform the state.
            if *self.cur_state.borrow() == State::StartState {
                self.trans_state(&self.command[self.command.len() - 1]);
            }
            *self.cur_index.borrow_mut() = self.command.len();
            self.store_token_and_trans_state();
        }

        // Add a EOF token to the end for the parser to determine the end of the command.
        *self.cur_state.borrow_mut() = State::EndState;
        self.tokens
            .borrow_mut()
            .push(Token::new(TokenType::Eof, ""));
    }

    // Store token and transform state.
    fn store_token_and_trans_state(&self) {
        // Move start index to end index for ready to read next token.
        self.move_start_index_to_next_non_blank_char();
        if *self.start_index.borrow() >= *self.cur_index.borrow() {
            return;
        }

        // Get the literal of token from char vector.
        let literal: String = self.command[*self.start_index.borrow()..*self.cur_index.borrow()]
            .iter()
            .collect();
        *self.start_index.borrow_mut() = *self.cur_index.borrow();

        // Match the state to get the token type.
        let token_type = self.trans_to_token_type();

        self.tokens
            .borrow_mut()
            .push(Token::new(token_type, &literal));

        // Judge whether the state should be reset or be end.
        if *self.start_index.borrow() < self.command.len() {
            // Reset lexer state
            self.trans_state(&self.command[*self.cur_index.borrow()]);
        }
    }

    fn move_start_index_to_next_non_blank_char(&self) {
        let mut index = self.start_index.borrow().clone();

        // Move index to next non blank char.
        while index < self.command.len() && self.command[index].is_whitespace() {
            index += 1;
        }

        // If index is out of range, we need to set it to the end of command.
        // It means from cur_index to the end of command are all blank chars.
        if index >= self.command.len() {
            index = self.command.len();
        }

        *self.start_index.borrow_mut() = index;
    }

    fn trans_to_literal_state_or_store_token(&self, c: &char) {
        if c.is_alphanumeric() || c.eq(&'_') {
            *self.cur_state.borrow_mut() = State::IdentState;
        } else {
            self.store_token_and_trans_state()
        }
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, c: &char) {
        if self.cur_state_is(State::EndState) {
            return;
        }

        match c {
            // =============== keywords ===============
            'w' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::WhileState1)
                }
            }
            'f' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::ForState1)
                }
            }
            'i' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::IfState1)
                }
            }
            'e' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::ElseState1)
                }
            }
            'b' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::BreakState1)
                }
            }
            'c' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::ContinueState1)
                }
            }
            'l' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::LetState1)
                }
            }
            'r' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::ReturnState1)
                }
            }
            's' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::StructState1)
                }
            }
            'n' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::NoneState1)
                }
            }
            't' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::TrueState1)
                }
            }

            // =============== number ===============
            '0'..='9' => {
                if self.cur_state_is(State::IdentState) {
                    self.set_state(State::IdentState)
                } else {
                    self.set_state(State::NumState)
                }
            }

            // =============== symbols ===============
            ',' => self.set_state(State::CommaState),
            '.' => self.set_state(State::DotState),
            ';' => self.set_state(State::SemiColonState),
            ':' => self.set_state(State::ColonState),
            '+' => self.set_state(State::PlusState),
            '-' => self.set_state(State::MinusState),
            '*' => self.set_state(State::StarState),
            '/' => self.set_state(State::SlashState),
            '%' => self.set_state(State::PercentState),
            '=' => self.set_state(State::AssignmentState),

            // =============== logical calculation ===============
            '!' => self.set_state(State::NotState),
            '>' => self.set_state(State::GreaterState),
            '<' => self.set_state(State::LessState),

            // =============== bit calculation ===============
            '&' => self.set_state(State::BitAndState),
            '|' => self.set_state(State::BitOrState),
            '~' => self.set_state(State::BitNotState),

            // =============== Others ===============
            '"' => self.set_state(State::QuoteState),
            '\'' => self.set_state(State::SingleQuoteState),
            '(' => self.set_state(State::LeftParenState),
            ')' => self.set_state(State::RightParenState),
            '{' => self.set_state(State::LeftBraceState),
            '}' => self.set_state(State::RightBraceState),
            '[' => self.set_state(State::LeftBracketState),
            ']' => self.set_state(State::RightBracketState),
            '_' => self.set_state(State::UnderscoreState),
            ' ' | '\n' => self.set_state(State::StartState),

            _ => self.set_state(State::IdentState),
        }
    }

    fn cur_state_is(&self, state: State) -> bool {
        *self.cur_state.borrow() == state
    }

    fn set_state(&self, state: State) {
        *self.cur_state.borrow_mut() = state;
    }

    fn trans_to_token_type(&self) -> TokenType {
        match *self.cur_state.borrow() {
            // =============== single symbols ===============
            State::CommaState => TokenType::Comma,
            State::DotState => TokenType::Dot,
            State::SemiColonState => TokenType::Semicolon,
            State::ColonState => TokenType::Colon,
            State::AssignmentState => TokenType::Assignment,
            State::LeftParenState => TokenType::LeftParen,
            State::RightParenState => TokenType::RightParen,
            State::LeftBraceState => TokenType::LeftBrace,
            State::RightBraceState => TokenType::RightBrace,
            State::LeftBracketState => TokenType::LeftBracket,
            State::RightBracketState => TokenType::RightBracket,
            State::QuoteState => TokenType::Quote,
            State::SingleQuoteState => TokenType::SingleQuote,

            // =============== logical calculation ===============
            State::NotState => TokenType::Not,
            State::GreaterState => TokenType::Greater,
            State::LessState => TokenType::Less,
            State::GreaterThanOrEqualToState => TokenType::GreaterThanOrEqualTo,
            State::LessThanOrEqualToState => TokenType::LessThanOrEqualTo,
            State::EqualToState => TokenType::EqualTo,
            State::NotEqualToState => TokenType::NotEqualTo,
            State::AndState => TokenType::And,
            State::OrState => TokenType::Or,

            // =============== bit calculation ===============
            State::BitAndState => TokenType::BitAnd,
            State::BitOrState => TokenType::BitOr,
            State::BitNotState => TokenType::BitNot,

            // =============== data calculate symbols ===============
            State::PlusState => TokenType::Plus,
            State::MinusState => TokenType::Minus,
            State::StarState => TokenType::Asterisk,
            State::SlashState => TokenType::Slash,
            State::PercentState => TokenType::Percent,
            State::PlusAssignState => TokenType::PlusAssign,
            State::MinusAssignState => TokenType::MinusAssign,
            State::StarAssignState => TokenType::AsteriskAssign,
            State::SlashAssignState => TokenType::SlashAssign,

            // =============== data ===============
            State::IdentState => TokenType::Ident,
            State::NumState => TokenType::IntegerNum,
            State::FloatNumState => TokenType::FloatNum,

            // =============== keywords ===============
            State::WhileState => TokenType::While,
            State::ForState => TokenType::For,
            State::IfState => TokenType::If,
            State::ElseState => TokenType::Else,
            State::BreakState => TokenType::Break,
            State::ContinueState => TokenType::Continue,
            State::LetState => TokenType::Let,
            State::FuncState => TokenType::Func,
            State::ReturnState => TokenType::Return,
            State::StructState => TokenType::Struct,
            State::EnumState => TokenType::Enum,
            State::NoneState => TokenType::None,
            State::TrueState => TokenType::True,
            State::FalseState => TokenType::False,

            _ => TokenType::Ident,
        }
    }
}
