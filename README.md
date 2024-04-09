# WocLang

The interpreter for this scripting language was originally written in **Go**, but has now been rewritten in **Rust**.

## Current development progress

- Complement the Lexer
  - Designed based on deterministic finite automata
  - Too few keywords, to be added later
- Complement the Parser
  - Parses `let, return` statements
  - Parses numeric literals (including `integers` and `floats`)
  - Parses prefix expressions (prefix support currently limited to `-` and `!`, with plans to add more support; implementation only for now)
  - Parses infix expressions (currently supports `+, -, *, /, >, <, >=, <=, ==, !=` infix operators)
  - Parses boolean literals (`true, false`)
  - Parses grouped expressions
  - Parses `if` expressions
  - Parses function expressions and function call expressions
- REPL
  - Adds REPL functionality, but evaluation is not yet possible due to the lack of syntax tree parsing