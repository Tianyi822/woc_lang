package parser

import (
	"testing"
	"woc_lang/ast"
	"woc_lang/lexer"
	"woc_lang/token"
)

type parserTestCase struct {
	input string
	num   int
	node  []ast.Node
}

func TestLetStatement(t *testing.T) {
	tests := []parserTestCase{
		{
			input: `var x = 822;
			var y = 701;
			var foo = 666;`,
			num: 3,
			node: []ast.Node{
				&ast.VarStatement{
					Token: token.Token{
						Type:    token.VAR,
						Literal: "var",
					},
					Name: ast.IdentExpression{
						Token: token.Token{
							Type:    token.IDENT,
							Literal: "x",
						},
						Value: "x",
					},
					Value: &ast.IntegerLiteral{
						Token: token.Token{
							Type:    token.NUM,
							Literal: "822",
						},
						Value: 822,
					},
				},
				&ast.VarStatement{
					Token: token.Token{
						Type:    token.VAR,
						Literal: "var",
					},
					Name: ast.IdentExpression{
						Token: token.Token{
							Type:    token.IDENT,
							Literal: "y",
						},
						Value: "y",
					},
					Value: &ast.IntegerLiteral{
						Token: token.Token{
							Type:    token.NUM,
							Literal: "701",
						},
						Value: 701,
					},
				},
				&ast.VarStatement{
					Token: token.Token{
						Type:    token.VAR,
						Literal: "var",
					},
					Name: ast.IdentExpression{
						Token: token.Token{
							Type:    token.IDENT,
							Literal: "foo",
						},
						Value: "foo",
					},
					Value: &ast.IntegerLiteral{
						Token: token.Token{
							Type:    token.NUM,
							Literal: "666",
						},
						Value: 666,
					},
				},
			},
		},
	}

	runParserTest(t, tests)
}

func TestReturnStatement(t *testing.T) {
	tests := []parserTestCase{
		{
			input: "return 666;",
			num:   1,
			node: []ast.Node{
				&ast.ReturnStatement{
					Token: token.Token{
						Type:    token.RETURN,
						Literal: "return",
					},
					ReturnValue: &ast.IntegerLiteral{
						Token: token.Token{
							Type:    token.NUM,
							Literal: "666",
						},
						Value: 666,
					},
				},
			},
		},
	}

	runParserTest(t, tests)
}

func runParserTest(t *testing.T, tests []parserTestCase) {
	t.Helper()

	for i, tt := range tests {
		l := lexer.New(tt.input)
		checkLexerErrors(t, l)

		parser := New(l)
		checkParserErrors(t, parser)

		if parser.program == nil {
			t.Fatalf("测试用例 %d 未解析到代码", i+1)
		}

		if len(parser.program.Statements) != tt.num {
			t.Fatalf("测试用例 %d 语法结构与预期不符:\n预期: %d\n实际: %d",
				i+1, tt.num, len(parser.program.Statements))
		}

		// TODO: 现在还没有对表达式进行解析，所以无法编写后续的测试逻辑，后续添加
	}

	t.Helper()
}

func checkParserErrors(t *testing.T, p *Parser) {
	errMessages := p.Errors()
	if len(errMessages) == 0 {
		return
	}

	t.Errorf("语法分析存在错误")
	for _, msg := range errMessages {
		t.Errorf("语法法分析错误: %q", msg)
	}
	t.FailNow()
}

func checkLexerErrors(t *testing.T, l *lexer.Lexer) {
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
