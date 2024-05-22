#[cfg(test)]
mod repl_test {
    use woc_lang::repl::{history::History, repl::REPL};

    #[test]
    fn test_run_repl() {
        let repl = REPL::new();

        repl.run();
    }

    #[test]
    fn test_history() {
        let hist = History::new();

        hist.add("1 + 2".to_string());
        hist.add("3 + 4".to_string());
        hist.add("5 + 6".to_string());

        assert_eq!(hist.get_last().unwrap(), "5 + 6");
        assert_eq!(hist.get_last().unwrap(), "3 + 4");
        assert_eq!(hist.get_last().unwrap(), "1 + 2");
        assert_eq!(hist.get_last().unwrap(), "5 + 6");

        assert_eq!(hist.get_next().unwrap(), "1 + 2");
        assert_eq!(hist.get_next().unwrap(), "3 + 4");
        assert_eq!(hist.get_next().unwrap(), "5 + 6");
        assert_eq!(hist.get_next().unwrap(), "1 + 2");
    }
}
