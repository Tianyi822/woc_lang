package lexer

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

	l := New(tt.input)
	tokens := analyseToken(l)

	for i, tok := range tt.expected {
		if tokens[i].Literal != tok.Literal || tokens[i].Type != tok.Type {
			t.Fatalf("生成 token 错误:\n预计得到: %v\n实际得到: %v",
				tt.expected[i], tok)
		}
	}
}

func TestReadStr(t *testing.T) {
	tests := struct {
		input       string
		expectedStr string
	}{
		"tt aa",
		"tt",
	}

	l := New(tests.input)

	readStr := l.readIdentifier()
	if readStr != tests.expectedStr {
		t.Fatalf("读取字符串功能错误:\n期望: %v\n实际: %v",
			tests.expectedStr, readStr)
	}
	if string(l.cur_rune) != "t" {
		t.Fatalf("词法分析器指针指向字符错误:\n期望: %v\n实际: %v\n指针所处位置为: %d",
			"t", string(l.cur_rune), l.cur_index)
	}
}

func TestReadNumStr(t *testing.T) {
	tests := []struct {
		input       string
		expectedStr string
	}{
		{
			"111222;",
			"111222",
		},
		{
			"111!222 333",
			"111",
		},
		{
			`111
			222`,
			"111",
		},
	}

	for _, tt := range tests {
		l := New(tt.input)

		numStr := l.readNumber()
		if numStr != tt.expectedStr {
			t.Fatalf("读取数字字符串错误:\n期望: %v\n实际: %v",
				tt.expectedStr, numStr)
		}
	}
}

func TestNextToken(t *testing.T) {
	tests := []struct {
		input          string
		expectedTokens []token.Token
	}{
		{
			"xasdf;",
			[]token.Token{
				{token.IDENT, "xasdf"},
				{token.SEMICOLON, ";"},
				{token.END_MARK, ""},
			},
		},
		{
			"822;",
			[]token.Token{
				{token.I32, "822"},
				{token.SEMICOLON, ";"},
				{token.END_MARK, ""},
			},
		},
		{
			"var x = 822;",
			[]token.Token{
				{token.VAR, "var"},
				{token.IDENT, "x"},
				{token.ASSIGN, "="},
				{token.I32, "822"},
				{token.SEMICOLON, ";"},
				{token.END_MARK, ""},
			},
		},
		{
			"var x = cty * 18 / 2 - 1;",
			[]token.Token{
				{token.VAR, "var"},
				{token.IDENT, "x"},
				{token.ASSIGN, "="},
				{token.IDENT, "cty"},
				{token.ASTERISK, "*"},
				{token.I32, "18"},
				{token.SLASH, "/"},
				{token.I32, "2"},
				{token.MINUS, "-"},
				{token.I32, "1"},
				{token.SEMICOLON, ";"},
				{token.END_MARK, ""},
			},
		},
		{
			`
			func main_cty_1() {
				return x + y;
			}
			`,
			[]token.Token{
				{token.FUNC, "func"},
				{token.IDENT, "main_cty_1"},
				{token.LPAREN, "("},
				{token.RPAREN, ")"},
				{token.LBRACE, "{"},
				{token.RETURN, "return"},
				{token.IDENT, "x"},
				{token.ADD, "+"},
				{token.IDENT, "y"},
				{token.SEMICOLON, ";"},
				{token.RBRACE, "}"},
				{token.END_MARK, ""},
			},
		},
	}

	for _, tt := range tests {
		l := New(tt.input)
		for _, expTok := range tt.expectedTokens {
			tok := l.NextToken()
			if tok.Literal != expTok.Literal || tok.Type != expTok.Type {
				t.Fatalf("token 错误:\n期望: %v\n实际: %v",
					expTok, tok)
			}
		}
	}
}

func analyseToken(l *Lexer) []token.Token {
	var tokens []token.Token

	for _, r := range l.code {
		tokens = append(
			tokens,
			token.Token{
				Type:    token.TokenType(r),
				Literal: string(r),
			},
		)
	}

	return tokens
}
