use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::lexer::state::State;
use crate::token::token::Token;
use crate::token::types::TokenType;

pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // Start index of token in command.
    start_index: Cell<usize>,

    // Current index of token vector.
    cur_index: Cell<usize>,

    // Store the tokens that are parsed.
    tokens: RefCell<Vec<Rc<Token>>>,

    // This is a key field to show the state about lexer at now.
    // It's used to define the type of the token currently.
    cur_state: Cell<State>,
}

/// This is a struct that is used to iterate the tokens.
pub struct TokensIter {
    tokens: Vec<Rc<Token>>,
    position: Cell<usize>,
}

impl TokensIter {
    pub fn next(&self) -> Option<Rc<Token>> {
        if self.position.get() >= self.tokens.len() {
            return Some(self.tokens[self.tokens.len() - 1].clone());
        }

        let token = self.tokens[self.position.get()].clone();
        self.position.set(self.position.get() + 1);

        Some(token)
    }
}

impl Lexer {
    /// Creates a new [`Lexer`].
    pub fn new(command: &str) -> Lexer {
        let l = Lexer {
            command: command.chars().collect(),
            start_index: Cell::new(0),
            cur_index: Cell::new(0),
            tokens: RefCell::new(Vec::new()),
            cur_state: Cell::new(State::StartState),
        };

        l.analyze_command();

        l
    }

    /// Creates a new [`LexerIter`].
    /// This is used to iterate the tokens.
    pub fn tokens_iter(self) -> TokensIter {
        TokensIter {
            tokens: self.tokens.into_inner(),
            position: Cell::new(0),
        }
    }

    // Clear the lexer data.
    pub fn clear(&self) {
        self.tokens.borrow_mut().clear();
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&self) {
        // Iterate the command char by char.
        for (index, c) in self.command.iter().enumerate() {
            // Update self.cur_index.
            self.cur_index.set(index);

            // If the char is blank, we need to store the token and transform the state.
            if c.is_whitespace() {
                if self.cur_state_is(State::StringState) {
                    self.set_state(State::StringState);
                } else {
                    self.store_token_and_trans_state();
                }
                continue;
            }

            let state = self.cur_state.get();
            match state {
                State::StartState => self.trans_state(c),

                // =============== keywords ===============
                // ============ while ============
                State::WhileState1 => {
                    if c.eq(&'h') {
                        self.set_state(State::WhileState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::WhileState2 => {
                    if c.eq(&'i') {
                        self.set_state(State::WhileState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::WhileState3 => {
                    if c.eq(&'l') {
                        self.set_state(State::WhileState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::WhileState4 => {
                    if c.eq(&'e') {
                        self.set_state(State::WhileState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
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
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ForState2 => {
                    if c.eq(&'r') {
                        self.set_state(State::ForState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ if ============
                State::IfState1 => {
                    if c.eq(&'f') {
                        self.set_state(State::IfState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ else ============
                State::ElseState1 => {
                    if c.eq(&'l') {
                        self.set_state(State::ElseState2);
                    } else if c.eq(&'n') {
                        self.set_state(State::EnumState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ElseState2 => {
                    if c.eq(&'s') {
                        self.set_state(State::ElseState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ElseState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::ElseState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ break ============
                State::BreakState1 => {
                    if c.eq(&'r') {
                        self.set_state(State::BreakState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::BreakState2 => {
                    if c.eq(&'e') {
                        self.set_state(State::BreakState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::BreakState3 => {
                    if c.eq(&'a') {
                        self.set_state(State::BreakState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::BreakState4 => {
                    if c.eq(&'k') {
                        self.set_state(State::BreakState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ continue ============
                State::ContinueState1 => {
                    if c.eq(&'o') {
                        self.set_state(State::ContinueState2)
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::ContinueState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState3 => {
                    if c.eq(&'t') {
                        self.set_state(State::ContinueState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState4 => {
                    if c.eq(&'i') {
                        self.set_state(State::ContinueState5);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState5 => {
                    if c.eq(&'n') {
                        self.set_state(State::ContinueState6);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState6 => {
                    if c.eq(&'u') {
                        self.set_state(State::ContinueState7);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ContinueState7 => {
                    if c.eq(&'e') {
                        self.set_state(State::ContinueState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ let ============
                State::LetState1 => {
                    if c.eq(&'e') {
                        self.set_state(State::LetState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::LetState2 => {
                    if c.eq(&'t') {
                        self.set_state(State::LetState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ function ============
                State::FuncState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::FuncState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::FuncState3 => {
                    if c.eq(&'c') {
                        self.set_state(State::FuncState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ return ============
                State::ReturnState1 => {
                    if c.eq(&'e') {
                        self.set_state(State::ReturnState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ReturnState2 => {
                    if c.eq(&'t') {
                        self.set_state(State::ReturnState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ReturnState3 => {
                    if c.eq(&'u') {
                        self.set_state(State::ReturnState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ReturnState4 => {
                    if c.eq(&'r') {
                        self.set_state(State::ReturnState5);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::ReturnState5 => {
                    if c.eq(&'n') {
                        self.set_state(State::ReturnState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ struct ============
                State::StructState1 => {
                    if c.eq(&'t') {
                        self.set_state(State::StructState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::StructState2 => {
                    if c.eq(&'r') {
                        self.set_state(State::StructState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::StructState3 => {
                    if c.eq(&'u') {
                        self.set_state(State::StructState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::StructState4 => {
                    if c.eq(&'c') {
                        self.set_state(State::StructState5);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::StructState5 => {
                    if c.eq(&'t') {
                        self.set_state(State::StructState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // ============ enum ============
                State::EnumState2 => {
                    if c.eq(&'u') {
                        self.set_state(State::EnumState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::EnumState3 => {
                    if c.eq(&'m') {
                        self.set_state(State::EnumState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // =============== none ===============
                State::NoneState1 => {
                    if c.eq(&'o') {
                        self.set_state(State::NoneState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::NoneState2 => {
                    if c.eq(&'n') {
                        self.set_state(State::NoneState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::NoneState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::NoneState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // =============== true ===============
                State::TrueState1 => {
                    if c.eq(&'r') {
                        self.set_state(State::TrueState2);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::TrueState2 => {
                    if c.eq(&'u') {
                        self.set_state(State::TrueState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::TrueState3 => {
                    if c.eq(&'e') {
                        self.set_state(State::TrueState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                // =============== false ===============
                State::FalseState2 => {
                    if c.eq(&'l') {
                        self.set_state(State::FalseState3);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::FalseState3 => {
                    if c.eq(&'s') {
                        self.set_state(State::FalseState4);
                    } else {
                        self.trans_2_ident_or_store_token(c);
                    }
                }

                State::FalseState4 => {
                    if c.eq(&'e') {
                        self.set_state(State::FalseState);
                    } else {
                        self.trans_2_ident_or_store_token(c);
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
                State::IntegerNumState => {
                    if c.is_numeric() || (state == State::IntegerNumState && c.eq(&'_')) {
                        self.set_state(State::IntegerNumState)
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

                // =============== string ===============
                State::StringState => {
                    if c.eq(&'"') {
                        self.store_token_and_trans_state()
                    } else if c.eq(&'\\') {
                        self.set_state(State::StringState);
                    } else if c.is_alphanumeric() || c.is_whitespace() {
                        self.set_state(State::StringState)
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
                | State::PercentState => {
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
                        self.set_state(State::IntegerNumState);
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

        // If the state of lexer is not end, we need to store the last token.
        if !self.cur_state_is(State::EndState) {
            // store the last state
            let last_state = self.cur_state.get();
            // Transform the state by current char.
            self.trans_state(&self.command[self.cur_index.get()]);

            // If the state is changed, we need to store the current state and recover the last state.
            if !self.cur_state_is(last_state) {
                let cur_state = self.cur_state.get();
                self.set_state(last_state);
                self.store_token_and_trans_state();
                self.set_state(cur_state);
            }

            // Store the last token.
            self.cur_index.set(self.command.len());
            self.store_token_and_trans_state();
        }

        // Add a EOF token to the end for the parser to determine the end of the command.
        self.cur_state.set(State::EndState);
        self.tokens
            .borrow_mut()
            .push(Rc::new(Token::new(TokenType::Eof, "")));
    }

    // Store token and transform state.
    fn store_token_and_trans_state(&self) {
        // Move start index to end index for ready to read next token.
        self.move_start_index_to_next_non_blank_char();
        if self.start_index.get() >= self.cur_index.get() {
            return;
        }

        // Match the state to get the token type.
        let token_type = self.trans_to_token_type();

        // Get the literal of token from char vector.
        let literal = match token_type {
            TokenType::String => {
                let literal: String = self.command
                    [self.start_index.get() + 1..self.cur_index.get()]
                    .iter()
                    .collect();

                // The reason we need to add 1 here is that the cur_index is at the second '"' character.
                // It should move to the next character to ensure the parsing of the next token.
                self.start_index.set(self.cur_index.get() + 1);

                literal
            }
            _ => {
                let literal: String = self.command[self.start_index.get()..self.cur_index.get()]
                    .iter()
                    .collect();

                self.start_index.set(self.cur_index.get());

                literal
            }
        };

        self.tokens
            .borrow_mut()
            .push(Rc::new(Token::new(token_type, &literal)));

        // Reset the state of lexer.
        self.set_state(State::StartState);

        // Judge whether the state should be reset or be ended.
        if self.start_index.get() < self.command.len() {
            // Reset lexer state
            self.trans_state(&self.command[self.start_index.get()]);
        }
    }

    fn move_start_index_to_next_non_blank_char(&self) {
        let mut index = self.start_index.get();

        // Move index to next non blank char.
        while index < self.command.len() && self.command[index].is_whitespace() {
            index += 1;
        }

        // If index is out of range, we need to set it to the end of command.
        // It means from cur_index to the end of command are all blank chars.
        if index >= self.command.len() {
            index = self.command.len();
        }

        self.start_index.set(index);
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
                } else if self.cur_state_is(State::IntegerNumState) {
                    self.set_state(State::IntegerNumState)
                } else if self.cur_state_is(State::FloatNumState) {
                    self.set_state(State::FloatNumState)
                } else {
                    self.set_state(State::IntegerNumState)
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
            '"' => self.set_state(State::StringState),
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
        self.cur_state.get() == state
    }

    fn set_state(&self, state: State) {
        self.cur_state.set(state);
    }

    fn trans_2_ident_or_store_token(&self, c: &char) {
        if c.is_alphanumeric() || c.eq(&'_') {
            self.set_state(State::IdentState)
        } else {
            self.store_token_and_trans_state();
        }
    }

    fn trans_to_token_type(&self) -> TokenType {
        match self.cur_state.get() {
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
            State::IntegerNumState => TokenType::IntegerNum,
            State::FloatNumState => TokenType::FloatNum,
            State::StringState => TokenType::String,

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
