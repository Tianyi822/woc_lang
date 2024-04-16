use std::io::{self, Write as _};

use crate::parser_v2::parser::Parser;

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

        let programs = p.programs();

        for program in programs {
            println!("{}", program);
        }
    }
}
