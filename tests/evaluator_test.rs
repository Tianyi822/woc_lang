#[cfg(test)]
mod evaluator_test {
    use woc_lang::{
        evaluator::evaluator::eval,
        object::object::{BaseValue, Object, Value},
        parser_v2::parser::Parser,
    };

    #[test]
    fn test_eval_prefix_exp() {
        let _tests = vec![
            ("!true;", false),
            ("!false;", true),
            ("!5;", false),
            ("!!true;", true),
            ("!!false;", false),
            ("!!5;", true),
        ];

        for (input, expected) in _tests {
            let evaluated = test_eval(input);
            test_equal_object(
                evaluated,
                Object::Base(BaseValue::Boolean(Value::new(expected))),
            );
        }
    }

    #[test]
    fn test_eval_integer_exp() {
        let _tests = vec![("5;", 5), ("10;", 10), ("-5;", -5), ("-10;", -10)];

        for (input, expected) in _tests {
            let value = test_eval(input);
            test_equal_object(
                value,
                Object::Base(BaseValue::Integer(Value::new(expected))),
            );
        }
    }

    #[test]
    fn test_eval_float_exp() {
        let _tests = vec![
            ("5.5;", 5.5),
            ("10.5;", 10.5),
            ("-5.5;", -5.5),
            ("-10.5;", -10.5),
        ];

        for (input, expected) in _tests {
            let evaluated = test_eval(input);
            test_equal_object(
                evaluated,
                Object::Base(BaseValue::Float(Value::new(expected))),
            );
        }
    }

    #[test]
    fn test_eval_boolean_exp() {
        let _tests = vec![("true;", true), ("false;", false)];

        for (input, expected) in _tests {
            let evaluated = test_eval(input);
            test_equal_object(
                evaluated,
                Object::Base(BaseValue::Boolean(Value::new(expected))),
            );
        }
    }

    fn test_eval(input: &str) -> Object {
        let parser = Parser::new(input);
        let program = parser.programs();

        return eval(program.get(0).unwrap());
    }

    fn test_equal_object(value: Object, expected: Object) {
        match (value, expected) {
            (Object::Base(BaseValue::Integer(v)), Object::Base(BaseValue::Integer(e))) => {
                assert_eq!(v.value(), e.value());
            }
            (Object::Base(BaseValue::Float(v)), Object::Base(BaseValue::Float(e))) => {
                assert_eq!(v.value(), e.value());
            }
            (Object::Base(BaseValue::Boolean(v)), Object::Base(BaseValue::Boolean(e))) => {
                assert_eq!(v.value(), e.value());
            }
            _ => panic!("Not implemented"),
        }
    }
}
