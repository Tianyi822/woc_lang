use std::cell::RefCell;

use crate::token::{Token, TokenType};

use self::state::State;

pub mod state;

#[derive(Debug)]
pub struct Lexer {
    // Command what user input.
    command: Vec<char>,

    // Start index of token in command.
    start_index: RefCell<usize>,

    // Store the tokens that are parsed.
    tokens: RefCell<Vec<Token>>,

    // This is a key field to show the state about lexer at now.
    // It's used to define the type of the token currently.
    cur_state: RefCell<State>,

    // Current index of token vector.
    // This field is used to iterate the tokens.
    position: RefCell<usize>,
}

impl Lexer {
    /// Creates a new [`Lexer`].
    pub fn new(command: &str) -> Lexer {
        let l = Lexer {
            command: command.chars().collect(),
            start_index: RefCell::new(0),
            tokens: RefCell::new(Vec::new()),
            cur_state: RefCell::new(State::StartState),
            position: RefCell::new(0),
        };

        l.analyze_command();

        l
    }

    // Get the tokens by range.
    pub fn joint_tokens_to_str_by_range(&self, start: u32, end: u32) -> String {
        let tokens = self.tokens.borrow();
        let mut result = String::new();

        // Iterate the tokens and get the literal of token.
        for i in start..end {
            result.push_str(tokens[i as usize].literal());
            if i < end - 1 {
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
    pub fn next_token(&self) -> Option<Token> {
        let tokens = self.tokens.borrow();
        let mut position = self.position.borrow_mut();

        if *position >= tokens.len() {
            return None;
        }

        let token = tokens[*position].clone();
        *position += 1;

        Some(token)
    }

    // Peek the next token.
    pub fn peek_token(&self) -> Option<Token> {
        let tokens = self.tokens.borrow();
        let position = self.position.borrow();
        if *position >= tokens.len() {
            return None;
        }

        let token = tokens[*position].clone();

        Some(token)
    }

    // Analyze the command and generate tokens.
    fn analyze_command(&self) {
        // Iterate the command char by char.
        for (index, c) in self.command.iter().enumerate() {
            let state = self.cur_state.borrow().clone();
            match state {
                State::StartState => self.trans_state(c),

                // =============== keywords ===============
                // ============ while ============
                State::WhileState1 => {
                    if c.eq(&'h') {
                        *(self.cur_state.borrow_mut()) = State::WhileState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::WhileState2 => {
                    if c.eq(&'i') {
                        *(self.cur_state.borrow_mut()) = State::WhileState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::WhileState3 => {
                    if c.eq(&'l') {
                        *(self.cur_state.borrow_mut()) = State::WhileState4;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::WhileState4 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::WhileState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ for ============
                State::ForState1 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::ForState2;
                    } else if c.eq(&'u') {
                        *(self.cur_state.borrow_mut()) = State::FuncState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ForState2 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::ForState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ if ============
                State::IfState1 => {
                    if c.eq(&'f') {
                        *(self.cur_state.borrow_mut()) = State::IfState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ else ============
                State::ElseState1 => {
                    if c.eq(&'l') {
                        *(self.cur_state.borrow_mut()) = State::ElseState2;
                    } else if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::EnumState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ElseState2 => {
                    if c.eq(&'s') {
                        *(self.cur_state.borrow_mut()) = State::ElseState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ElseState3 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::ElseState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ break ============
                State::BreakState1 => {
                    if c.eq(&'r') {
                        *(self.cur_state.borrow_mut()) = State::BreakState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::BreakState2 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::BreakState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::BreakState3 => {
                    if c.eq(&'a') {
                        *(self.cur_state.borrow_mut()) = State::BreakState4;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::BreakState4 => {
                    if c.eq(&'k') {
                        *(self.cur_state.borrow_mut()) = State::BreakState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ continue ============
                State::ContinueState1 => {
                    if c.eq(&'o') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState2 => {
                    if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState3 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState4;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState4 => {
                    if c.eq(&'i') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState5;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState5 => {
                    if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState6;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState6 => {
                    if c.eq(&'u') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState7;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ContinueState7 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::ContinueState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ let ============
                State::LetState1 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::LetState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::LetState2 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::LetState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ function ============
                State::FuncState2 => {
                    if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::FuncState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::FuncState3 => {
                    if c.eq(&'c') {
                        *(self.cur_state.borrow_mut()) = State::FuncState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ return ============
                State::ReturnState1 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::ReturnState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ReturnState2 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::ReturnState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ReturnState3 => {
                    if c.eq(&'u') {
                        *(self.cur_state.borrow_mut()) = State::ReturnState4;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ReturnState4 => {
                    if c.eq(&'r') {
                        *(self.cur_state.borrow_mut()) = State::ReturnState5;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::ReturnState5 => {
                    if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::ReturnState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ struct ============
                State::StructState1 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::StructState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::StructState2 => {
                    if c.eq(&'r') {
                        *(self.cur_state.borrow_mut()) = State::StructState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::StructState3 => {
                    if c.eq(&'u') {
                        *(self.cur_state.borrow_mut()) = State::StructState4;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::StructState4 => {
                    if c.eq(&'c') {
                        *(self.cur_state.borrow_mut()) = State::StructState5;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::StructState5 => {
                    if c.eq(&'t') {
                        *(self.cur_state.borrow_mut()) = State::StructState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // ============ enum ============
                State::EnumState2 => {
                    if c.eq(&'u') {
                        *(self.cur_state.borrow_mut()) = State::EnumState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::EnumState3 => {
                    if c.eq(&'m') {
                        *(self.cur_state.borrow_mut()) = State::EnumState;
                    } else {
                        self.trans_state(c);
                    }
                }

                // =============== none ===============
                State::NoneState1 => {
                    if c.eq(&'o') {
                        *(self.cur_state.borrow_mut()) = State::NoneState2;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::NoneState2 => {
                    if c.eq(&'n') {
                        *(self.cur_state.borrow_mut()) = State::NoneState3;
                    } else {
                        self.trans_state(c);
                    }
                }

                State::NoneState3 => {
                    if c.eq(&'e') {
                        *(self.cur_state.borrow_mut()) = State::NoneState;
                    } else {
                        self.trans_state(c);
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
                | State::NoneState => self.store_token_and_trans_state(index, c),

                // =============== number ===============
                State::NumState => {
                    if c.is_numeric() || (state == State::NumState && c.eq(&'_')) {
                        *(self.cur_state.borrow_mut()) = State::NumState;
                    } else if c.is_alphabetic() {
                        *(self.cur_state.borrow_mut()) = State::LiteralState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== Literal ===============
                State::LiteralState => {
                    if !c.is_ascii() {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== white space ===============
                State::WhiteSpaceState => {
                    self.trans_state(c);
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
                    self.store_token_and_trans_state(index, c);
                }

                // =============== combined symbols ===============
                State::AssignmentState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::EqualToState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::GreaterState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::GreaterThanOrEqualToState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::LessState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::LessThanOrEqualToState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::NotState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::NotEqualToState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::BitAndState => {
                    if c.eq(&'&') {
                        *(self.cur_state.borrow_mut()) = State::AndState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::BitOrState => {
                    if c.eq(&'|') {
                        *(self.cur_state.borrow_mut()) = State::OrState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::PlusState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::PlusEqualState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::MinusState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::MinusEqualState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::StarState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::StarEqualState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::SlashState => {
                    if c.eq(&'=') {
                        *(self.cur_state.borrow_mut()) = State::SlashEqualState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                State::AndState
                | State::OrState
                | State::EqualToState
                | State::NotEqualToState
                | State::GreaterThanOrEqualToState
                | State::LessThanOrEqualToState
                | State::PlusEqualState
                | State::MinusEqualState
                | State::StarEqualState
                | State::SlashEqualState => {
                    self.store_token_and_trans_state(index, c);
                }

                // ============ Underline ============
                State::UnderscoreState => {
                    if c.is_alphabetic() || c.is_numeric() {
                        *(self.cur_state.borrow_mut()) = State::LiteralState;
                    } else {
                        self.store_token_and_trans_state(index, c);
                    }
                }

                // =============== end ===============
                State::EndState => break,
            }
        }

        // If the lexer's state is not end, we need to store the last token.
        let state = self.cur_state.borrow().clone();
        if state != State::EndState {
            // Determine if the state is 'start' to ensure completion of the last token parsing.
            // If it's 'start', change the state accordingly;
            // otherwise, store the last token with the current state.
            //
            // For the string "&&&", the first token is "&&" and the second is "&".
            // When parsing the last "&" token, the state reverts to 'start' before "&&" is stored.
            // Therefore, it's necessary to adjust the state appropriately and store the "&" token.
            if state == State::StartState {
                self.trans_state(&self.command[self.start_index.borrow().clone()]);
            }
            self.store_token_and_trans_state(self.command.len(), &' ');
        }
    }

    // Store token and transform state.
    fn store_token_and_trans_state(&self, cur_index: usize, cur_char: &char) {
        let state = self.cur_state.borrow().clone();
        let mut start_index = self.start_index.borrow_mut();

        // Move start index to end index for ready to read next token.
        *start_index = self.move_index_to_next_non_blank_char(*start_index);

        // Get the literal of token from char vector.
        let literal: String = self.command[*start_index..cur_index].iter().collect();
        *start_index = cur_index;

        if !(state == State::WhiteSpaceState) {
            // Match the state to get the token type.
            let token_type = match state {
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
                State::StarState => TokenType::Star,
                State::SlashState => TokenType::Slash,
                State::PercentState => TokenType::Percent,
                State::PlusEqualState => TokenType::PlusEqual,
                State::MinusEqualState => TokenType::MinusEqual,
                State::StarEqualState => TokenType::StarEqual,
                State::SlashEqualState => TokenType::SlashEqual,

                // =============== data ===============
                State::LiteralState => TokenType::Literal,
                State::NumState => TokenType::Num,

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

                _ => todo!(),
            };

            self.tokens
                .borrow_mut()
                .push(Token::new(token_type, &literal));
        }

        // Judge whether the state should be reset or be end.
        if *start_index < self.command.len() {
            // Reset lexer state
            // *state = State::Start;
            self.trans_state(cur_char);
        } else {
            *start_index = self.command.len() - 1;
            *self.cur_state.borrow_mut() = State::EndState;

            // Add a EOF token to the end for the parser to determine the end of the command.
            self.tokens
                .borrow_mut()
                .push(Token::new(TokenType::Eof, ""));
        }
    }

    fn move_index_to_next_non_blank_char(&self, cur_index: usize) -> usize {
        let mut index = cur_index;

        // Move index to next non blank char.
        while index < self.command.len() && self.command[index].is_whitespace() {
            index += 1;
        }

        // If index is out of range, we need to set it to the end of command.
        // It means from cur_index to the end of command are all blank chars.
        if index >= self.command.len() {
            index = self.command.len();
        }

        index
    }

    // Transform lexer state by the current char.
    fn trans_state(&self, c: &char) {
        // Get state and cur_index, and update them by the current char.
        let mut state = self.cur_state.borrow_mut();

        if *state == State::EndState {
            return;
        }

        if c.is_whitespace() {
            *state = State::WhiteSpaceState;
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

            _ => *state = State::LiteralState,
        }
    }
}
