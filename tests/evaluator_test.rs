#[cfg(test)]
mod evaluator_test {
    use std::vec;

    use woc_lang::{
        evaluator,
        object::object::{Object, OBJType},
        parser::parser::Parser,
    };

    #[test]
    fn test_integer_eval() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            // ("-5", -5),
            // ("-10", -10),
            // ("5 + 5 + 5 + 5 - 10", 10),
            // ("2 * 2 * 2 * 2 * 2", 32),
            // ("-50 + 100 + -50", 0),
            // ("5 * 2 + 10", 20),
            // ("5 + 2 * 10", 25),
            // ("20 + 2 * -10", 0),
            // ("50 / 2 * 2 + 10", 60),
            // ("2 * (5 + 10)", 30),
            // ("3 * 3 * 3 + 10", 37),
            // ("3 * (3 * 3) + 10", 37),
            // ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for (input, expected) in tests {
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
        }
    }

    fn test_eval(input: &str) -> Box<dyn Object> {
        let parser = Parser::new(input);
        let program = parser.program();

        return evaluator::evaluator::eval(program);
    }

    fn test_integer_object(obj: Box<dyn Object>, expected: i64) {
        match obj.obj_type() {
            OBJType::IntegerObj => {
                let result = obj.inspect().parse::<i64>().unwrap();
                assert_eq!(result, expected);
            }
            _ => panic!("object is not Integer. got={:?}", obj.obj_type()),
        }
    }
}
