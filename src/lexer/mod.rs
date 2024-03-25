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
                        *self.cur_state.borrow_mut() = State::WhileState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::WhileState2 => {
                    if c.eq(&'i') {
                        *self.cur_state.borrow_mut() = State::WhileState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::WhileState3 => {
                    if c.eq(&'l') {
                        *self.cur_state.borrow_mut() = State::WhileState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::WhileState4 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::WhileState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ for ============
                State::ForState1 => {
                    if c.eq(&'o') {
                        *self.cur_state.borrow_mut() = State::ForState2;
                    } else if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::FuncState2;
                    } else if c.eq(&'a') {
                        *self.cur_state.borrow_mut() = State::FalseState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ForState2 => {
                    if c.eq(&'r') {
                        *self.cur_state.borrow_mut() = State::ForState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ if ============
                State::IfState1 => {
                    if c.eq(&'f') {
                        *self.cur_state.borrow_mut() = State::IfState;
                    } else {
                        self.trans_to_literal_state_or_store_token(c);
                    }
                }

                // ============ else ============
                State::ElseState1 => {
                    if c.eq(&'l') {
                        *self.cur_state.borrow_mut() = State::ElseState2;
                    } else if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::EnumState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ElseState2 => {
                    if c.eq(&'s') {
                        *self.cur_state.borrow_mut() = State::ElseState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ElseState3 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::ElseState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ break ============
                State::BreakState1 => {
                    if c.eq(&'r') {
                        *self.cur_state.borrow_mut() = State::BreakState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::BreakState2 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::BreakState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::BreakState3 => {
                    if c.eq(&'a') {
                        *self.cur_state.borrow_mut() = State::BreakState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::BreakState4 => {
                    if c.eq(&'k') {
                        *self.cur_state.borrow_mut() = State::BreakState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ continue ============
                State::ContinueState1 => {
                    if c.eq(&'o') {
                        *self.cur_state.borrow_mut() = State::ContinueState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState2 => {
                    if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::ContinueState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState3 => {
                    if c.eq(&'t') {
                        *self.cur_state.borrow_mut() = State::ContinueState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState4 => {
                    if c.eq(&'i') {
                        *self.cur_state.borrow_mut() = State::ContinueState5;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState5 => {
                    if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::ContinueState6;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState6 => {
                    if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::ContinueState7;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ContinueState7 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::ContinueState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ let ============
                State::LetState1 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::LetState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::LetState2 => {
                    if c.eq(&'t') {
                        *self.cur_state.borrow_mut() = State::LetState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ function ============
                State::FuncState2 => {
                    if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::FuncState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::FuncState3 => {
                    if c.eq(&'c') {
                        *self.cur_state.borrow_mut() = State::FuncState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ return ============
                State::ReturnState1 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::ReturnState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ReturnState2 => {
                    if c.eq(&'t') {
                        *self.cur_state.borrow_mut() = State::ReturnState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ReturnState3 => {
                    if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::ReturnState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ReturnState4 => {
                    if c.eq(&'r') {
                        *self.cur_state.borrow_mut() = State::ReturnState5;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::ReturnState5 => {
                    if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::ReturnState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ struct ============
                State::StructState1 => {
                    if c.eq(&'t') {
                        *self.cur_state.borrow_mut() = State::StructState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::StructState2 => {
                    if c.eq(&'r') {
                        *self.cur_state.borrow_mut() = State::StructState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::StructState3 => {
                    if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::StructState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::StructState4 => {
                    if c.eq(&'c') {
                        *self.cur_state.borrow_mut() = State::StructState5;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::StructState5 => {
                    if c.eq(&'t') {
                        *self.cur_state.borrow_mut() = State::StructState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // ============ enum ============
                State::EnumState2 => {
                    if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::EnumState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::EnumState3 => {
                    if c.eq(&'m') {
                        *self.cur_state.borrow_mut() = State::EnumState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // =============== none ===============
                State::NoneState1 => {
                    if c.eq(&'o') {
                        *self.cur_state.borrow_mut() = State::NoneState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::NoneState2 => {
                    if c.eq(&'n') {
                        *self.cur_state.borrow_mut() = State::NoneState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::NoneState3 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::NoneState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // =============== true ===============
                State::TrueState1 => {
                    if c.eq(&'r') {
                        *self.cur_state.borrow_mut() = State::TrueState2;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::TrueState2 => {
                    if c.eq(&'u') {
                        *self.cur_state.borrow_mut() = State::TrueState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::TrueState3 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::TrueState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                // =============== false ===============
                State::FalseState2 => {
                    if c.eq(&'l') {
                        *self.cur_state.borrow_mut() = State::FalseState3;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::FalseState3 => {
                    if c.eq(&'s') {
                        *self.cur_state.borrow_mut() = State::FalseState4;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    }
                }

                State::FalseState4 => {
                    if c.eq(&'e') {
                        *self.cur_state.borrow_mut() = State::FalseState;
                    } else {
                        *self.cur_state.borrow_mut() = State::IdentState;
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
                        *self.cur_state.borrow_mut() = State::NumState;
                    } else if c.eq(&'.') {
                        *self.cur_state.borrow_mut() = State::FloatNumState;
                    } else if c.is_alphabetic() {
                        *self.cur_state.borrow_mut() = State::IdentState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::FloatNumState => {
                    if c.is_numeric() {
                        *self.cur_state.borrow_mut() = State::FloatNumState;
                    } else if c.is_alphabetic() {
                        *self.cur_state.borrow_mut() = State::IdentState;
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
                        *self.cur_state.borrow_mut() = State::EqualToState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::GreaterState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::GreaterThanOrEqualToState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::LessState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::LessThanOrEqualToState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::NotState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::NotEqualToState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::BitAndState => {
                    if c.eq(&'&') {
                        *self.cur_state.borrow_mut() = State::AndState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::BitOrState => {
                    if c.eq(&'|') {
                        *self.cur_state.borrow_mut() = State::OrState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::PlusState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::PlusAssignState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::MinusState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::MinusAssignState;
                    } else if c.is_numeric() {
                        *self.cur_state.borrow_mut() = State::NumState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::StarState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::StarAssignState;
                    } else {
                        self.store_token_and_trans_state();
                    }
                }

                State::SlashState => {
                    if c.eq(&'=') {
                        *self.cur_state.borrow_mut() = State::SlashAssignState;
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
                        *self.cur_state.borrow_mut() = State::IdentState;
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
            *self.cur_state.borrow_mut() = State::LiteralState;
        } else {
            self.store_token_and_trans_state()
        }
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, c: &char) {
        // Get state and cur_index, and update them by the current char.
        let mut state = self.cur_state.borrow_mut();

        if *state == State::EndState {
            return;
        }

        match c {
            // =============== keywords ===============
            'w' => *state = State::WhileState1,
            'f' => *state = State::ForState1,
            'i' => *state = State::IfState1,
            'e' => *state = State::ElseState1,
            'b' => *state = State::BreakState1,
            'c' => *state = State::ContinueState1,
            'l' => *state = State::LetState1,
            'r' => *state = State::ReturnState1,
            's' => *state = State::StructState1,
            'n' => *state = State::NoneState1,
            't' => *state = State::TrueState1,

            // =============== number ===============
            '0'..='9' => *state = State::NumState,

            // =============== symbols ===============
            ',' => *state = State::CommaState,
            '.' => *state = State::DotState,
            ';' => *state = State::SemiColonState,
            ':' => *state = State::ColonState,
            '+' => *state = State::PlusState,
            '-' => *state = State::MinusState,
            '*' => *state = State::StarState,
            '/' => *state = State::SlashState,
            '%' => *state = State::PercentState,
            '=' => *state = State::AssignmentState,

            // =============== logical calculation ===============
            '!' => *state = State::NotState,
            '>' => *state = State::GreaterState,
            '<' => *state = State::LessState,

            // =============== bit calculation ===============
            '&' => *state = State::BitAndState,
            '|' => *state = State::BitOrState,
            '~' => *state = State::BitNotState,

            // =============== Others ===============
            '"' => *state = State::QuoteState,
            '\'' => *state = State::SingleQuoteState,
            '(' => *state = State::LeftParenState,
            ')' => *state = State::RightParenState,
            '{' => *state = State::LeftBraceState,
            '}' => *state = State::RightBraceState,
            '[' => *state = State::LeftBracketState,
            ']' => *state = State::RightBracketState,
            '_' => *state = State::UnderscoreState,
            ' ' | '\n' => *state = State::StartState,

            _ => *state = State::LiteralState,
        }
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
