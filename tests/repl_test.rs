#[cfg(test)]
mod repl_test {
    use woc_lang::repl::REPL;

    #[test]
    fn test_run_repl() {
        let repl = REPL::new();

        repl.run();
    }
}
