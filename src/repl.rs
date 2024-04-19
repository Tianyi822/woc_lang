use std::io::{self, Write as _};

use crate::{
    evaluator::evaluator,
    parser_v2::parser::Parser,
};

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

        let evaluated = evaluator::eval(programs.get(0).unwrap());
        if !evaluated.is_null() {
            println!("{}", evaluated);
        }
    }
}
