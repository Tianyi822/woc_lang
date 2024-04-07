use std::io::{self, Write as _};

use crate::parser::parser::Parser;

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

        let p = Parser::new(&input);

        if p.errors.borrow().len() > 0 {
            for error in p.errors.borrow().iter() {
                println!("error: {}", error);
            }
            continue;
        }

        for stmt in p.program.statements.borrow().iter() {
            println!("{}", stmt.to_string());
        }
    }
}
