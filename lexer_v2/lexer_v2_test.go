package lexer_v2

import (
	"testing"
	"woc_lang/token_v2"
)

func TestNextToken(t *testing.T) {
	tests := []struct {
		input          string
		expectedTokens []token_v2.Token
	}{
		{
			",.;: _ + * / ( ) {}[]",
			[]token_v2.Token{
				{token_v2.COMMA, ","},
				{token_v2.DOT, "."},
				{token_v2.SEMICOLON, ";"},
				{token_v2.COLON, ":"},
				{token_v2.UNDERLINE, "_"},
				{token_v2.ADD, "+"},
				{token_v2.ASTERISK, "*"},
				{token_v2.SLASH, "/"},
				{token_v2.LPAREN, "("},
				{token_v2.RPAREN, ")"},
				{token_v2.LBRACE, "{"},
				{token_v2.RBRACE, "}"},
				{token_v2.LBRACKET, "["},
				{token_v2.RBRACKET, "]"},
			},
		},
		{
			"= == ! != > >=  < <=  & &&  |  || >> << - ->",
			[]token_v2.Token{
				{token_v2.ASSIGN, "="},
				{token_v2.EQ, "=="},
				{token_v2.BANG, "!"},
				{token_v2.NEQ, "!="},
				{token_v2.GT, ">"},
				{token_v2.GE, ">="},
				{token_v2.LT, "<"},
				{token_v2.LE, "<="},
				{token_v2.BIT_AND, "&"},
				{token_v2.AND, "&&"},
				{token_v2.BIT_OR, "|"},
				{token_v2.OR, "||"},
				{token_v2.BIT_R_OFFSET, ">>"},
				{token_v2.BIT_L_OFFSET, "<<"},
				{token_v2.MINUS, "-"},
				{token_v2.ARROW, "->"},
				{token_v2.EOF, ""},
			},
		},
	}

	for _, tt := range tests {
		l := New(tt.input)
		checkParserErrors(t, l)

		for i, expTok := range tt.expectedTokens {
			tok := l.NextToken()
			if tok.Literal != expTok.Literal || tok.Type != expTok.Type {
				t.Fatalf("第 %d 个 token 解析错误:\n期望: %v\n实际: %v",
					i+1, expTok, tok)
			}
		}
	}
}

func checkParserErrors(t *testing.T, l *Lexer) {
	errors := l.Errors()
	if len(errors) == 0 {
		return
	}

	t.Errorf("词法分析器存在 %d 个错误", len(errors))
	for _, msg := range errors {
		t.Errorf("词法分析错误: %q", msg)
	}
	t.FailNow()
}
