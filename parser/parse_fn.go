package parser

// 存放用于解析不同 Token 所需要的前缀解析函数和中缀解析函数

import (
	"fmt"
	"strconv"
	"woc_lang/ast"
	"woc_lang/token"
)

type (
	// 前缀语法解析对应的方法
	prefixParseFn func() ast.Expression

	// 中缀语法解析对应的方法，所以会在形参中多一个 ast.Expression AST 节点
	// 这个参数是用于解析中缀运算符左侧的内容
	infixParseFn func(ast.Expression) ast.Expression
)

func RegisterParseFns(p *Parser) {
	p.registerPrefix(token.IDENT, p.parseIdentExpression)
	// TODO: 按道理说这里应该传入一个 parseNumExpression，但现在主要是先实现功能，就全部默认整型了
	p.registerPrefix(token.NUM, p.parseIntegerLiteral)
}

// parseIdentifier 解析标识符表达式语法
func (p *Parser) parseIdentExpression() ast.Expression {
	return &ast.IdentExpression{
		Token: p.cur_token,
		Value: p.cur_token.Literal,
	}
}

// parseIntegerLiteral 解析整型字面量
func (p *Parser) parseIntegerLiteral() ast.Expression {
	integerLiteral := &ast.IntegerLiteral{
		Token: p.cur_token,
	}

	intNum, err := strconv.ParseInt(p.cur_token.Literal, 0, 64)
	if err != nil {
		msg := fmt.Sprintf("数值字符串字面量转整型错误，字面量为: %v\n错误信息: %s",
			integerLiteral.TokenLiteral(), err)
		p.errors = append(p.errors, msg)
		return nil
	}

	integerLiteral.Value = intNum

	return integerLiteral
}

// registerPrefix 注册前缀处理方法
func (p *Parser) registerPrefix(tokenType token.TokenType, fn prefixParseFn) {
	p.prefixParseFns[tokenType] = fn
}

// registerInfix 注册中缀处理方法
func (p *Parser) registerInfix(tokenType token.TokenType, fn infixParseFn) {
	p.infixParseFns[tokenType] = fn
}
