mod parser_test {
    use woc_lang::parser::Parser;

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let parser = Parser::new(input);

        if parser.program.statements.borrow().len() != 1 {
            panic!(
                "parser.program.statements does not contain 1 statements. got={}",
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
                "parser.program.statements does not contain 4 statements. got={}",
                parser.program.statements.borrow().len()
            );
        }

        let results = vec!["5", "10", "5.1", "10"];

        let mut i = 0;
        for stmt in parser.program.statements.borrow().iter() {
            let exp = stmt.to_string();
            if exp != results[i] {
                panic!(
                    "parser.program.statements[{}] does not contain {}. got={}",
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
                "parser.program.statements does not contain 3 statements. got={}",
                parser.program.statements.borrow().len()
            );
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

        if parser.program.statements.borrow().len() != 3 {
            panic!(
                "parser.program.statements does not contain 3 statements. got={}",
                parser.program.statements.borrow().len()
            );
        }
    }

    #[test]
    fn test_let_stmt_without_semi() {
        let input = "
            let x 666
            let = 666
            let 666 777
        ";

        let parser = Parser::new(input);

        assert_eq!(parser.errors.borrow().len(), 8);
        for error in parser.errors.borrow().iter() {
            println!("{}", error);
        }
    }
}
