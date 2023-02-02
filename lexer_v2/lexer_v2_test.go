package lexer_v2

import (
	"testing"
	"woc_lang/token_v2"
)

type lexerTestCase struct {
	input          string
	expectedTokens []token_v2.Token
}

func TestNextToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			",.;:+*/(){}[]",
			[]token_v2.Token{
				{token_v2.COMMA, ","},
				{token_v2.DOT, "."},
				{token_v2.SEMICOLON, ";"},
				{token_v2.COLON, ":"},
				{token_v2.ADD, "+"},
				{token_v2.ASTERISK, "*"},
				{token_v2.SLASH, "/"},
				{token_v2.LPAREN, "("},
				{token_v2.RPAREN, ")"},
				{token_v2.LBRACE, "{"},
				{token_v2.RBRACE, "}"},
				{token_v2.LBRACKET, "["},
				{token_v2.RBRACKET, "]"},
				{token_v2.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestKeyWorkToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			`
				var ;
				bool true false;
				func if else meth return int32;`,
			[]token_v2.Token{
				{token_v2.VAR, "var"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.BOOL, "bool"},
				{token_v2.TRUE, "true"},
				{token_v2.FALSE, "false"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.FUNC, "func"},
				{token_v2.IF, "if"},
				{token_v2.ELSE, "else"},
				{token_v2.METH, "meth"},
				{token_v2.RETURN, "return"},
				{token_v2.INT32, "int32"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestIdentToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			"cty foo view bar test icu egg money rust test_1 test_tt;",
			[]token_v2.Token{
				{token_v2.IDENT, "cty"},
				{token_v2.IDENT, "foo"},
				{token_v2.IDENT, "view"},
				{token_v2.IDENT, "bar"},
				{token_v2.IDENT, "test"},
				{token_v2.IDENT, "icu"},
				{token_v2.IDENT, "egg"},
				{token_v2.IDENT, "money"},
				{token_v2.IDENT, "rust"},
				{token_v2.IDENT, "test_1"},
				{token_v2.IDENT, "test_tt"},
				{token_v2.SEMICOLON, ";"},
			},
		},
		{
			"tt tt_t tt_3_5_tt;",
			[]token_v2.Token{
				{token_v2.IDENT, "tt"},
				{token_v2.IDENT, "tt_t"},
				{token_v2.IDENT, "tt_3_5_tt"},
				{token_v2.SEMICOLON, ";"},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestPreSymbolToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			"= == ! != > >=  < <=  & &&  |  || >> << - -> _tt;",
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
				{token_v2.IDENT, "_tt"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestNumToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			"822; 701;",
			[]token_v2.Token{
				{token_v2.NUM, "822"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.NUM, "701"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func runLexerTest(t *testing.T, tests []lexerTestCase) {
	t.Helper()
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
	errTokens := l.Errors()
	if len(errTokens) == 0 {
		return
	}

	t.Errorf("词法分析器存在 %d 个错误", len(errTokens))
	for _, errTok := range errTokens {
		t.Errorf("(%s) 词法分析错误: %q", errTok.Literal, errTok.Msg)
	}
	t.FailNow()
}
