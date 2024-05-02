#[cfg(test)]
mod parser_test {
    use woc_lang::parser_v2::parser::Parser;

    #[test]
    fn test_func_and_call() {
        let input = "
            func add(x, y) { return x + y; }
            add(5, 5);
        ";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 2);

        let func_stmt = programs.get(0).unwrap();
        assert_eq!(func_stmt.to_string(), "func add(x, y) {return (x + y);}");

        let call_exp = programs.get(1).unwrap();
        assert_eq!(call_exp.to_string(), "add(5, 5)");
    }

    #[test]
    fn test_parse_multi_prefix() {
        let input = "!!true;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let multi_prefix = programs.get(0).unwrap();
        assert_eq!(multi_prefix.to_string(), "!!true");
    }

    #[test]
    fn test_parse_let_stmt() {
        let input = "let x = 822;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let let_stmt = programs.get(0).unwrap();
        assert_eq!(let_stmt.to_string(), "let x = 822;");
    }

    #[test]
    fn test_parse_return_stmt() {
        let input = "return 822;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let return_stmt = programs.get(0).unwrap();
        assert_eq!(return_stmt.to_string(), "return 822;");
    }

    #[test]
    fn test_parse_identifier_exp() {
        let input = "foobar;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let identifier_exp = programs.get(0).unwrap();
        assert_eq!(identifier_exp.to_string(), "foobar");
    }

    #[test]
    fn test_parse_prefix_exp() {
        let input = "!822; -x;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 2);

        let not_exp = programs.get(0).unwrap();
        assert_eq!(not_exp.to_string(), "!822");

        let minus_exp = programs.get(1).unwrap();
        assert_eq!(minus_exp.to_string(), "-x");
    }

    #[test]
    fn test_parse_group_exp() {
        let input = "822;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let group_exp = programs.get(0).unwrap();
        assert_eq!(group_exp.to_string(), "822");
    }

    #[test]
    fn test_parse_boolean_exp() {
        let input = "true; false;";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 2);

        let true_exp = programs.get(0).unwrap();
        assert_eq!(true_exp.to_string(), "true");

        let false_exp = programs.get(1).unwrap();
        assert_eq!(false_exp.to_string(), "false");
    }

    #[test]
    fn test_parse_infix_exp() {
        let input = "
            5 + 5; 
            5 - 5; 
            5 * 5; 
            5 / 5; 
            5 == 5; 
            5 != 5; 
            5 < 5; 
            5 > 5; 
            5 <= 5; 
            5 >= 5; 
            true && false; 
            true || false;
        ";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 12);

        let infix_exp = programs.get(0).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 + 5)");

        let infix_exp = programs.get(1).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 - 5)");

        let infix_exp = programs.get(2).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 * 5)");

        let infix_exp = programs.get(3).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 / 5)");

        let infix_exp = programs.get(4).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 == 5)");

        let infix_exp = programs.get(5).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 != 5)");

        let infix_exp = programs.get(6).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 < 5)");

        let infix_exp = programs.get(7).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 > 5)");

        let infix_exp = programs.get(8).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 <= 5)");

        let infix_exp = programs.get(9).unwrap();
        assert_eq!(infix_exp.to_string(), "(5 >= 5)");

        let infix_exp = programs.get(10).unwrap();
        assert_eq!(infix_exp.to_string(), "(true && false)");

        let infix_exp = programs.get(11).unwrap();
        assert_eq!(infix_exp.to_string(), "(true || false)");
    }

    #[test]
    fn test_parse_block_stmt() {
        let input = "
            {
                let x = 5;
                let y = 10;
                let foobar = 838383;
            }
        ";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let block_stmt = programs.get(0).unwrap();
        assert_eq!(
            block_stmt.to_string(),
            "{let x = 5; let y = 10; let foobar = 838383;}"
        );
    }

    #[test]
    fn test_func_stmt() {
        let input = "func add(x, y) { return x + y; }";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let func_stmt = programs.get(0).unwrap();
        assert_eq!(func_stmt.to_string(), "func add(x, y) {return (x + y);}");
    }

    #[test]
    fn test_parse_if_exp() {
        let input = "if (x < y) { return x; }";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let if_exp = programs.get(0).unwrap();
        assert_eq!(if_exp.to_string(), "if (x < y) {return x;}");
    }

    #[test]
    fn test_parse_if_else_exp() {
        let input = "if (x < y) { return x; } else if (x == y) { return x; } else { return x; }";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let if_else_exp = programs.get(0).unwrap();
        assert_eq!(
            if_else_exp.to_string(),
            "if (x < y) {return x;} else if (x == y) {return x;} else {return x;}"
        );
    }

    #[test]
    fn test_parse_call_exp() {
        let input = "add(5, 5);";

        let parser = Parser::new(input);
        let programs = parser.programs();

        assert_eq!(programs.len(), 1);

        let call_exp = programs.get(0).unwrap();
        assert_eq!(call_exp.to_string(), "add(5, 5)");
    }
}
