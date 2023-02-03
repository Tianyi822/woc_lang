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
				func if else meth return ;`,
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
			[]token_v2.Token{
				{token_v2.IDENT, "t"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "tr"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "tru"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "tt"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "f"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "fa"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "fal"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "fals"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "ffa"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "fu"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "fun"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "v"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "va"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "i"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "e"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "el"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "els"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "m"},
				{token_v2.IDENT, "me"},
				{token_v2.IDENT, "met"},
				{token_v2.IDENT, "r"},
				{token_v2.IDENT, "re"},
				{token_v2.IDENT, "ret"},
				{token_v2.IDENT, "retu"},
				{token_v2.IDENT, "retur"},
				{token_v2.IDENT, "x"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "b"},
				{token_v2.IDENT, "bo"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "boo"},
				{token_v2.SEMICOLON, ";"},
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
				{token_v2.IDENT, "true_test"},
				{token_v2.IDENT, "false_test"},
				{token_v2.IDENT, "func_test"},
				{token_v2.IDENT, "var_test"},
				{token_v2.IDENT, "if_test"},
				{token_v2.IDENT, "else_test"},
				{token_v2.IDENT, "meth_test"},
				{token_v2.IDENT, "return_test"},
				{token_v2.IDENT, "bool_3_test"},
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

func TestCodeTokens(t *testing.T) {
	tests := []lexerTestCase{
		{
			`
			var arr_1 = [1, 2, 3, 4];
			len(arr_1);
			arr_1.first();
			out(arr_1[1]);
			`,
			[]token_v2.Token{
				{token_v2.VAR, "var"},
				{token_v2.IDENT, "arr_1"},
				{token_v2.ASSIGN, "="},
				{token_v2.LBRACKET, "["},
				{token_v2.NUM, "1"},
				{token_v2.COMMA, ","},
				{token_v2.NUM, "2"},
				{token_v2.COMMA, ","},
				{token_v2.NUM, "3"},
				{token_v2.COMMA, ","},
				{token_v2.NUM, "4"},
				{token_v2.RBRACKET, "]"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "len"},
				{token_v2.LPAREN, "("},
				{token_v2.IDENT, "arr_1"},
				{token_v2.RPAREN, ")"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "arr_1"},
				{token_v2.DOT, "."},
				{token_v2.IDENT, "first"},
				{token_v2.LPAREN, "("},
				{token_v2.RPAREN, ")"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IDENT, "out"},
				{token_v2.LPAREN, "("},
				{token_v2.IDENT, "arr_1"},
				{token_v2.LBRACKET, "["},
				{token_v2.NUM, "1"},
				{token_v2.RBRACKET, "]"},
				{token_v2.RPAREN, ")"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.EOF, ""},
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
			[]token_v2.Token{
				{token_v2.FUNC, "func"},
				{token_v2.IDENT, "test1"},
				{token_v2.LPAREN, "("},
				{token_v2.RPAREN, ")"},
				{token_v2.ARROW, "->"},
				{token_v2.BOOL, "bool"},
				{token_v2.LBRACE, "{"},
				{token_v2.BOOL, "bool"},
				{token_v2.IDENT, "flag"},
				{token_v2.ASSIGN, "="},
				{token_v2.NUM, "1"},
				{token_v2.EQ, "=="},
				{token_v2.NUM, "1"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.IF, "if"},
				{token_v2.LPAREN, "("},
				{token_v2.IDENT, "flag"},
				{token_v2.RPAREN, ")"},
				{token_v2.LBRACE, "{"},
				{token_v2.RETURN, "return"},
				{token_v2.TRUE, "true"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.RBRACE, "}"},
				{token_v2.ELSE, "else"},
				{token_v2.LBRACE, "{"},
				{token_v2.RETURN, "return"},
				{token_v2.FALSE, "false"},
				{token_v2.SEMICOLON, ";"},
				{token_v2.RBRACE, "}"},
				{token_v2.RBRACE, "}"},
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
