package parser

import (
	"fmt"
	"testing"
	"woc_lang/ast"
	"woc_lang/lexer"
)

func TestParsingVarStatement(t *testing.T) {
	tests := []struct {
		input              string
		expectedIdentifier string
		expectedValue      any
	}{
		{"var x = 5;", "x", 5},
		{"var foobar = y;", "foobar", "y"},
	}

	for _, tt := range tests {
		l := lexer.New(tt.input)
		checkLexerErrors(t, l)

		parser := New(l)
		checkParserErrors(t, parser)

		if len(parser.program.Statements) != 1 {
			t.Fatalf("语句解析错误，解析语句数量为: %d", len(parser.program.Statements))
		}

		stmt := parser.program.Statements[0]
		if !testVarStatement(t, stmt, tt.expectedIdentifier) {
			return
		}

		val := stmt.(*ast.VarStatement).Value
		if !testLiteralExpression(t, val, tt.expectedValue) {
			return
		}
	}
}

func TestParsingPrefixExpressions(t *testing.T) {
	prefixTests := []struct {
		input    string
		Operator string
		value    any
	}{
		{"!foobar;", "!", "foobar"},
		{"-5;", "-", 5},
	}

	for _, tt := range prefixTests {
		l := lexer.New(tt.input)
		checkLexerErrors(t, l)

		parser := New(l)
		checkParserErrors(t, parser)

		if parser.program == nil {
			t.Fatalf("测试用例未解析到代码")
		}

		if len(parser.program.Statements) != 1 {
			t.Fatalf("测试用例语法结构与预期不符:\n预期: %d\n实际: %d",
				1, len(parser.program.Statements))
		}

		stmt, ok := parser.program.Statements[0].(*ast.ExpressionStatement)
		if !ok {
			t.Fatalf("语句解析错误: %T", parser.program.Statements[0])
		}

		preExp, ok := stmt.Expression.(*ast.PrefixExpression)
		if !ok {
			t.Fatalf("该语句并非前缀表达式: %T", stmt.Expression)
		}

		if preExp.Operator != tt.Operator {
			t.Fatalf("该语句操作符错误\n期望: '%s'\n, 实际得到: '%s'",
				tt.Operator,
				preExp.Operator)
		}

		if !testLiteralExpression(t, preExp.Right, tt.value) {
			return
		}
	}
}

func testVarStatement(t *testing.T, s ast.Statement, name string) bool {
	if s.TokenLiteral() != "var" {
		t.Errorf("s.TokenLiteral 不是 'var', 实际得到: %T", s.TokenLiteral())
		return false
	}

	varStmt, ok := s.(*ast.VarStatement)
	if !ok {
		t.Errorf("语句并非 *ast.LetStatement 类型，实际得到: %T", s)
		return false
	}

	if varStmt.Name.Value != name {
		t.Errorf("varStmt.Name.Value 期望得到: '%s'，实际得到: %s", name, varStmt.Name.Value)
		return false
	}

	if varStmt.Name.TokenLiteral() != name {
		t.Errorf("varStmt.Name.TokenLiteral() 期望得到: '%s'，got=%s",
			name, varStmt.Name.TokenLiteral())
		return false
	}

	return true
}

func testLiteralExpression(t *testing.T, exp ast.Expression, expected any) bool {
	switch v := expected.(type) {
	case int:
		return testIntegerLiteral(t, exp, int64(v))
	case int64:
		return testIntegerLiteral(t, exp, v)
	case string:
		return testIdentifier(t, exp, v)
	}
	t.Errorf("无法处理该类型: %T", exp)
	return false
}

func testIntegerLiteral(t *testing.T, il ast.Expression, value int64) bool {
	integ, ok := il.(*ast.IntegerLiteral)
	if !ok {
		t.Errorf("传入字面量类型不是 *ast.IntegerLiteral: %T", il)
		return false
	}

	if integ.Value != value {
		t.Errorf("integ.Value 期望是 %d. 实际=%d", value, integ.Value)
		return false
	}

	if integ.TokenLiteral() != fmt.Sprintf("%d", value) {
		t.Errorf("integ.TokenLiteral 期望是 %d. 实际=%s", value,
			integ.TokenLiteral())
		return false
	}

	return true
}

func testIdentifier(t *testing.T, exp ast.Expression, value string) bool {
	ident, ok := exp.(*ast.IdentExpression)
	if !ok {
		t.Errorf("exp 实际得到结果类型并不是 *ast.Identifier，实际得到=%T", exp)
		return false
	}

	if ident.Value != value {
		t.Errorf("ident.Value 期望得到 %s. 实际得到: %s", value, ident.Value)
		return false
	}

	if ident.TokenLiteral() != value {
		t.Errorf("ident.TokenLiteral 期望得到 %s. 实际得到: %s", value, ident.TokenLiteral())
		return false
	}

	return true
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
