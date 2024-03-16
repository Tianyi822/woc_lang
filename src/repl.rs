use std::io::{self, Write as _};

use crate::lexer::Lexer;

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

        while let token = lexer.next_token() {
            println!("{:?}", token);
        }
    }
}
