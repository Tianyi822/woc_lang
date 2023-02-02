package parser

import "woc_lang/lexer_v2"

type Parser struct {
	l      *lexer_v2.Lexer
	errors []string
}
