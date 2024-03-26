use std::io::{self, Write as _};

use crate::lexer::lexer::Lexer;
use crate::token::token::TokenType;

pub fn run() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let lexer = Lexer::new(&input);

        loop {
            let token = lexer.next_token();
            if token.token_type() == &TokenType::Eof {
                break;
            }
            println!("{:?}", token);
        }
    }
}
