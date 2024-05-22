use std::io::Write as _;

use crate::evaluator_v2::evaluator::Evaluator;

use super::history::History;

pub struct REPL {
    history: History,
    evaluator: Evaluator,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            evaluator: Evaluator::new(None),
        }
    }

    fn deal_input(&self) {}

    pub fn run(&self) {
        loop {
            let mut code = String::new();

            print!("> ");
            std::io::stdout().flush().unwrap();

            // Get input from user
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" {
                self.history.clean();
                break;
            }

            while input.trim().ends_with('\\') {
                code += &input[..input.len() - 2];

                // Clear the input buffer
                input.clear();
                // Get next line
                print!("> ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();
            }

            code += &input;

            let p = crate::parser_v2::parser::Parser::new(&code);

            let programs = p.programs();

            let evaluated = self.evaluator.eval(programs.get(0).unwrap());
            if !evaluated.is_null() {
                println!("{}", evaluated);
            }
        }
    }
}
