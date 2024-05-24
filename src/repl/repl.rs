use std::io::Write as _;

use crate::evaluator_v2::evaluator::Evaluator;
use crate::parser_v2::parser::Parser;

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

    fn deal_input(&self, input: String) -> String {
        match input.trim() {
            ":exit" => {
                self.history.clean();
                std::process::exit(0);
            }
            // up arrow
            "\u{1b}[A" => match self.history.get_last() {
                Some(last) => last,
                None => String::new(),
            },
            // down arrow
            "\u{1b}[B" => match self.history.get_next() {
                Some(next) => next,
                None => String::new(),
            },
            _ => {
                let mut input_buf = input.clone();
                let mut code = input_buf.clone();

                while input_buf.trim().ends_with('\\') {
                    code += &input_buf[..input.len() - 2];

                    // Clear the input buffer
                    input_buf.clear();
                    // Get next line
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut input_buf).unwrap();
                }

                self.history.add(&code);

                code
            }
        }
    }

    pub fn run(&self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();

            // Get input from user
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let input = self.deal_input(input);

            let p = Parser::new(&input);

            let programs = p.programs();

            let evaluated = self.evaluator.eval(programs.get(0).unwrap());
            if !evaluated.is_null() {
                println!("{}", evaluated);
            }
        }
    }
}
