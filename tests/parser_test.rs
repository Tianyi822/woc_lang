#[cfg(test)]
mod parser_test {
    use woc_lang::parser::parser::Parser;

    #[test]
    fn test_call_expression() {
        let input = "
            add(1, 2 * 3, 4 + 5);
            add(1, 2); 
            add(1);
            add();
            add(x, y);
            add(x, y, 1);
        ";

        let p = Parser::new(input);

        if p.program.statements.borrow().len() != 6 {
            panic!(
                "parser.program.statements does not contain 5 statements. got = {}",
                p.program.statements.borrow().len()
            );
        }

        let results = vec![
            "add(1, (2 * 3), (4 + 5))",
            "add(1, 2)",
            "add(1)",
            "add()",
            "add(x, y)",
            "add(x, y, 1)",
        ];

        let mut i = 0;
        for stmt in p.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_fn_expression() {
        let input = "func add (x, y) { x + y; }";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        // Assert the statement is an FnExp.
        let results = vec!["func add (x, y) { (x + y); }"];
        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_else_if_expression() {
        let input = "if (x < y) { x; y; z; } else if (x > y) { a; b; c; } else if (x == y) { d; e; f; } else { g; h; i; }";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        // Assert the statement is an IfElseIfExp.
        let results = vec!["if (x < y) { x; y; z; } else if (x > y) { a; b; c; } else if (x == y) { d; e; f; } else { g; h; i; }"];
        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x; y; z; } else { a; b; c; }";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        // Assert the statement is an IfElseExp.
        let results = vec!["if (x < y) { x; y; z; } else { a; b; c; }"];
        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x; y; z; }";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        // Assert the statement is an IfExp.
        let results = vec!["if (x < y) { x; y; z; }"];
        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b;", "((-a) * b)"),
            ("!-a;", "(!(-a))"),
            ("a + b + c;", "((a + b) + c)"),
            ("a + b - c;", "((a + b) - c)"),
            ("a * b * c;", "((a * b) * c)"),
            ("a * b / c;", "((a * b) / c)"),
            ("a + b / c;", "(a + (b / c))"),
            ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4;", "(3 + 4)"),
            ("5 > 4 == 3 < 4;", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4;", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true;", "true"),
            ("false;", "false"),
            ("3 > 5 == false;", "((3 > 5) == false)"),
            ("3 < 5 == true;", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4;", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2;", "((5 + 5) * 2)"),
            ("2 / (5 + 5);", "(2 / (5 + 5))"),
            ("-(5 + 5);", "(-(5 + 5))"),
            ("!(true == true);", "(!(true == true))"),
            // ("a + add(b * c) + d;", "((a + add((b * c))) + d)"),
        ];

        for tt in tests.iter() {
            let input = tt.0;
            let expected = tt.1;

            let parser = Parser::new(input);
            for stmt in parser.program.statements.borrow().iter() {
                let exp = stmt.to_string();
                assert_eq!(exp, expected, "expected={}, got={}", expected, exp);
            }
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "
            5 + 5;
            5 - 5;
            5 * 5;
            5 / 5;
            5 > 5;
            5 < 5;
            5 == 5;
            5 != 5;
            true == true;
            true != false;
            false == false;
            -a * b;
        ";

        let parser = Parser::new(input);

        let results = vec![
            "(5 + 5)",
            "(5 - 5)",
            "(5 * 5)",
            "(5 / 5)",
            "(5 > 5)",
            "(5 < 5)",
            "(5 == 5)",
            "(5 != 5)",
            "(true == true)",
            "(true != false)",
            "(false == false)",
            "((-a) * b)",
        ];

        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_prefix_expression() {
        let input = "!5; -15; -x;";

        let parser = Parser::new(input);

        // "-15;" was parsed as a IntegerNum token, so it will not be parsed as a number expression.
        let results = vec!["(!5)", "-15", "(-x)"];
        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }
    }

    #[test]
    fn test_integer_and_float_number_expression() {
        let input = "
            5;
            10;
            5.1;
            10.0;
        ";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 4 {
            panic!(
                "parser.program.statements does not contain 4 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        let results = vec!["5", "10", "5.1", "10"];

        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_return_stmt() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 3 {
            panic!(
                "parser.program.statements does not contain 3 statements. got = {}",
                parser.program.statements.borrow().len()
            );
        }

        let results = vec!["return 5;", "return 10;", "return 993322;"];

        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got = {}",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }

    #[test]
    fn test_let_stmt() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let parser = Parser::new(input);

        let results = vec!["let x = 5;", "let y = 10;", "let foobar = 838383;"];

        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain \"{}\". got = \"{}\"",
                    i, results[i], exp
                );
            }
            i += 1;
        }
    }
}
