package lexer_v2

import (
	"testing"
	"woc_lang/token_v2"
)

func TestToken(t *testing.T) {
	tt := struct {
		input          string
		expectedTokens []token_v2.Token
	}{
		",.;:_",
		[]token_v2.Token{
			{token_v2.COMMA, ","},
			{token_v2.DOT, "."},
			{token_v2.SEMICOLON, ";"},
			{token_v2.COLON, ":"},
			//{token_v2.LPAREN, "("},
			//{token_v2.RPAREN, ")"},
			//{token_v2.LBRACKET, "["},
			//{token_v2.RBRACKET, "]"},
			//{token_v2.LBRACE, "{"},
			//{token_v2.RBRACE, "}"},
		},
	}

	l := New(tt.input)

	for i, expTok := range tt.expectedTokens {
		tok := l.NextToken()
		if tok.Literal != expTok.Literal || tok.Type != expTok.Type {
			t.Fatalf("第 %d 个 token 解析错误:\n期望: %v\n实际: %v",
				i+1, expTok, tok)
		}
	}
}
