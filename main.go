package main

import (
	"os"
	"os/user"
	"woc_lang/repl"
)

func main() {
	u, err := user.Current()
	if err != nil {
		panic(err)
	}

	repl.StartREPL(u, os.Stdin, os.Stdout)
}
