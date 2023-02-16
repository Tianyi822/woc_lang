package lexer

import (
	"testing"
	"woc_lang/token"
)

type lexerTestCase struct {
	input          string
	expectedTokens []token.Token
}

func TestNextToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			",.;:+*/(){}[]",
			[]token.Token{
				{token.COMMA, ","},
				{token.DOT, "."},
				{token.SEMICOLON, ";"},
				{token.COLON, ":"},
				{token.ADD, "+"},
				{token.ASTERISK, "*"},
				{token.SLASH, "/"},
				{token.LPAREN, "("},
				{token.RPAREN, ")"},
				{token.LBRACE, "{"},
				{token.RBRACE, "}"},
				{token.LBRACKET, "["},
				{token.RBRACKET, "]"},
				{token.EOF, ""},
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
				func if else meth return ;`,
			[]token.Token{
				{token.VAR, "var"},
				{token.SEMICOLON, ";"},
				{token.BOOL, "bool"},
				{token.TRUE, "true"},
				{token.FALSE, "false"},
				{token.SEMICOLON, ";"},
				{token.FUNC, "func"},
				{token.IF, "if"},
				{token.ELSE, "else"},
				{token.METH, "meth"},
				{token.RETURN, "return"},
				{token.SEMICOLON, ";"},
				{token.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestIdentToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			`t; tr; tru; tt;
			f; fa; fal; fals; ffa; fu; fun;
			v; va;
			i;
			e; el; els;
			m me met
			 r re ret retu retur x;
			b bo; boo;
			cty foo view bar test icu egg money rust test_1 test_tt;
			true_test false_test func_test var_test
			if_test else_test meth_test return_test bool_3_test;`,
			[]token.Token{
				{token.IDENT, "t"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "tr"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "tru"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "tt"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "f"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "fa"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "fal"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "fals"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "ffa"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "fu"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "fun"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "v"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "va"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "i"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "e"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "el"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "els"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "m"},
				{token.IDENT, "me"},
				{token.IDENT, "met"},
				{token.IDENT, "r"},
				{token.IDENT, "re"},
				{token.IDENT, "ret"},
				{token.IDENT, "retu"},
				{token.IDENT, "retur"},
				{token.IDENT, "x"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "b"},
				{token.IDENT, "bo"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "boo"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "cty"},
				{token.IDENT, "foo"},
				{token.IDENT, "view"},
				{token.IDENT, "bar"},
				{token.IDENT, "test"},
				{token.IDENT, "icu"},
				{token.IDENT, "egg"},
				{token.IDENT, "money"},
				{token.IDENT, "rust"},
				{token.IDENT, "test_1"},
				{token.IDENT, "test_tt"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "true_test"},
				{token.IDENT, "false_test"},
				{token.IDENT, "func_test"},
				{token.IDENT, "var_test"},
				{token.IDENT, "if_test"},
				{token.IDENT, "else_test"},
				{token.IDENT, "meth_test"},
				{token.IDENT, "return_test"},
				{token.IDENT, "bool_3_test"},
				{token.SEMICOLON, ";"},
			},
		},
		{
			"tt tt_t tt_3_5_tt;",
			[]token.Token{
				{token.IDENT, "tt"},
				{token.IDENT, "tt_t"},
				{token.IDENT, "tt_3_5_tt"},
				{token.SEMICOLON, ";"},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestPreSymbolToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			"= == ! != > >=  < <=  & &&  |  || >> << - -> _tt;",
			[]token.Token{
				{token.ASSIGN, "="},
				{token.EQ, "=="},
				{token.BANG, "!"},
				{token.NEQ, "!="},
				{token.GT, ">"},
				{token.GE, ">="},
				{token.LT, "<"},
				{token.LE, "<="},
				{token.BIT_AND, "&"},
				{token.AND, "&&"},
				{token.BIT_OR, "|"},
				{token.OR, "||"},
				{token.BIT_R_OFFSET, ">>"},
				{token.BIT_L_OFFSET, "<<"},
				{token.MINUS, "-"},
				{token.ARROW, "->"},
				{token.IDENT, "_tt"},
				{token.SEMICOLON, ";"},
				{token.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestNumToken(t *testing.T) {
	tests := []lexerTestCase{
		{
			"822; 701;",
			[]token.Token{
				{token.NUM, "822"},
				{token.SEMICOLON, ";"},
				{token.NUM, "701"},
				{token.SEMICOLON, ";"},
				{token.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func TestCodeTokens(t *testing.T) {
	tests := []lexerTestCase{
		{
			`
			var arr_1 = [1, 2, 3, 4];
			len(arr_1);
			arr_1.first();
			out(arr_1[1]);
			`,
			[]token.Token{
				{token.VAR, "var"},
				{token.IDENT, "arr_1"},
				{token.ASSIGN, "="},
				{token.LBRACKET, "["},
				{token.NUM, "1"},
				{token.COMMA, ","},
				{token.NUM, "2"},
				{token.COMMA, ","},
				{token.NUM, "3"},
				{token.COMMA, ","},
				{token.NUM, "4"},
				{token.RBRACKET, "]"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "len"},
				{token.LPAREN, "("},
				{token.IDENT, "arr_1"},
				{token.RPAREN, ")"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "arr_1"},
				{token.DOT, "."},
				{token.IDENT, "first"},
				{token.LPAREN, "("},
				{token.RPAREN, ")"},
				{token.SEMICOLON, ";"},
				{token.IDENT, "out"},
				{token.LPAREN, "("},
				{token.IDENT, "arr_1"},
				{token.LBRACKET, "["},
				{token.NUM, "1"},
				{token.RBRACKET, "]"},
				{token.RPAREN, ")"},
				{token.SEMICOLON, ";"},
				{token.EOF, ""},
			},
		},
		{
			`
			func test1() -> bool {
				bool flag = 1 == 1;
				if (flag) {
					return true;
				} else {
					return false;
				}
			}
			`,
			[]token.Token{
				{token.FUNC, "func"},
				{token.IDENT, "test1"},
				{token.LPAREN, "("},
				{token.RPAREN, ")"},
				{token.ARROW, "->"},
				{token.BOOL, "bool"},
				{token.LBRACE, "{"},
				{token.BOOL, "bool"},
				{token.IDENT, "flag"},
				{token.ASSIGN, "="},
				{token.NUM, "1"},
				{token.EQ, "=="},
				{token.NUM, "1"},
				{token.SEMICOLON, ";"},
				{token.IF, "if"},
				{token.LPAREN, "("},
				{token.IDENT, "flag"},
				{token.RPAREN, ")"},
				{token.LBRACE, "{"},
				{token.RETURN, "return"},
				{token.TRUE, "true"},
				{token.SEMICOLON, ";"},
				{token.RBRACE, "}"},
				{token.ELSE, "else"},
				{token.LBRACE, "{"},
				{token.RETURN, "return"},
				{token.FALSE, "false"},
				{token.SEMICOLON, ";"},
				{token.RBRACE, "}"},
				{token.RBRACE, "}"},
				{token.EOF, ""},
			},
		},
	}

	runLexerTest(t, tests)
}

func runLexerTest(t *testing.T, tests []lexerTestCase) {
	t.Helper()
	for _, tt := range tests {
		l := New(tt.input)
		checkLexerErrors(t, l)

		for i, expTok := range tt.expectedTokens {
			tok := l.NextToken()
			if tok.Literal != expTok.Literal || tok.Type != expTok.Type {
				t.Fatalf("第 %d 个 token 解析错误:\n期望: %v\n实际: %v",
					i+1, expTok, tok)
			}
		}
	}
}

func checkLexerErrors(t *testing.T, l *Lexer) {
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
