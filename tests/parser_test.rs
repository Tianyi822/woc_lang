mod parser_test {
    use woc_lang::parser::Parser;

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
}
