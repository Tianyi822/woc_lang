package lexer_v2

import (
	"testing"
	"woc_lang/token"
)

func TestToken(t *testing.T) {
	tt := struct {
		input    string
		expected []token.Token
	}{
		",.;:()[]{}|",
		[]token.Token{
			{token.COMMA, ","},
			{token.DOT, "."},
			{token.SEMICOLON, ";"},
			{token.COLON, ":"},
			{token.LPAREN, "("},
			{token.RPAREN, ")"},
			{token.LBRACKET, "["},
			{token.RBRACKET, "]"},
			{token.LBRACE, "{"},
			{token.RBRACE, "}"},
			{token.OR, "|"},
		},
	}

	New(tt.input)
}
