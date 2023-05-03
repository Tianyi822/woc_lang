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
	p.registerPrefix(token.IDENT, p.parseIdentLiteral)
	// TODO: 按道理说这里应该传入一个 parseNumExpression，但现在主要是先实现功能，就全部默认整型了
	p.registerPrefix(token.NUM, p.parseIntegerLiteral)
	p.registerPrefix(token.TRUE, p.parseBooleanLiteral)
	p.registerPrefix(token.FALSE, p.parseBooleanLiteral)
	p.registerPrefix(token.BANG, p.parsePrefixExpression)
	p.registerPrefix(token.MINUS, p.parsePrefixExpression)
	p.registerPrefix(token.LPAREN, p.parseGroupExpression)
	p.registerPrefix(token.IF, p.parseIfExpression)
	p.registerPrefix(token.FUNC, p.parseFunctionLiteral)

	p.registerInfix(token.ADD, p.parseInfixExpression)
	p.registerInfix(token.MINUS, p.parseInfixExpression)
	p.registerInfix(token.ASTERISK, p.parseInfixExpression)
	p.registerInfix(token.SLASH, p.parseInfixExpression)
	p.registerInfix(token.GT, p.parseInfixExpression)
	p.registerInfix(token.LT, p.parseInfixExpression)
	p.registerInfix(token.GE, p.parseInfixExpression)
	p.registerInfix(token.LE, p.parseInfixExpression)
	p.registerInfix(token.EQ, p.parseInfixExpression)
	p.registerInfix(token.NEQ, p.parseInfixExpression)
	p.registerInfix(token.LPAREN, p.parseCallExpression)
}

// ============================ parse literal start ============================

// parseIdentifier 解析标识符表达式语法
func (p *Parser) parseIdentLiteral() ast.Expression {
	return &ast.IdentLiteral{
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

// parseBooleanLiteral 解析布尔值字面量
func (p *Parser) parseBooleanLiteral() ast.Expression {
	return &ast.BooleanLiteral{
		Token: p.cur_token,
		Value: p.cur_token.Literal == "true",
	}
}

// ============================ parse literal end ============================

// parsePrefixExpression 解析前缀表达式
func (p *Parser) parsePrefixExpression() ast.Expression {
	preExp := &ast.PrefixExpression{
		Token:    p.cur_token,
		Operator: p.cur_token.Literal,
	}

	p.nextToken()

	preExp.Right = p.parseExpression(PREFIX_LEVEL)

	return preExp
}

// parseInfixExpression 解析中缀表达式
func (p *Parser) parseInfixExpression(left ast.Expression) ast.Expression {
	exp := &ast.InfixExpression{
		Token:    p.cur_token,
		Operator: p.cur_token.Literal,
		LeftExp:  left,
	}

	priority := p.curPriority()
	p.nextToken()
	exp.RightExp = p.parseExpression(priority)

	return exp
}

// parseCallExpression 解析函数调用表达式
func (p *Parser) parseCallExpression(left ast.Expression) ast.Expression {
	funcName, ok := left.(*ast.IdentLiteral)
	if !ok {
		p.statementErrorf("函数调用语法错误，函数名 Token 类型错误，错误 Token 为: %T", left)
		return nil
	}

	exp := &ast.CallExpression{
		Token:        p.cur_token,
		FunctionName: funcName,
		Arguments:    p.parseCallArguments(),
	}

	return exp
}

// parseCallArguments 解析函数调用的实参列表
func (p *Parser) parseCallArguments() []ast.Expression {
	var args []ast.Expression

	if p.expectPeek(token.RPAREN) {
		return args
	}

	p.nextToken()
	args = append(args, p.parseExpression(LEVEL_0))

	for p.expectPeek(token.COMMA) {
		p.nextToken()
		args = append(args, p.parseExpression(LEVEL_0))
	}

	if !p.expectPeek(token.RPAREN) {
		p.statementErrorf("函数调用语法错误，错误 Token 为: %s", p.cur_token.Literal)
		return nil
	}

	return args
}

// parseGroupExpression 解析分组表达式
func (p *Parser) parseGroupExpression() ast.Expression {
	p.nextToken()

	exp := p.parseExpression(LEVEL_0)

	if !p.expectPeek(token.RPAREN) {
		return nil
	}

	return exp
}

// parseIfExpression 解析 if 表达式
func (p *Parser) parseIfExpression() ast.Expression {
	ifExp := &ast.IfExpression{
		Token: p.cur_token,
	}

	if !p.expectPeek(token.LPAREN) {
		p.statementErrorf("if 语句格式错误，条件语句左侧没有括号: '(")
		return nil
	}

	p.nextToken()
	ifExp.Condition = p.parseExpression(LEVEL_0)
	if ifExp.Condition == nil {
		p.statementErrorf("if 语句格式错误，没有条件判断")
	}

	if !p.expectPeek(token.RPAREN) {
		p.statementErrorf("if 语句格式错误，条件语句右侧没有括号: ')")
		return nil
	}

	if !p.expectPeek(token.LBRACE) {
		p.statementErrorf("if 语句格式错误，代码块缺少左花括号: '{")
		return nil
	}

	ifExp.Consequence = p.parseBlockStatement()

	if p.expectPeek(token.ELSE) {
		ifExp.ElseExpression = p.parseElseExpression()

	}

	return ifExp
}

func (p *Parser) parseElseExpression() *ast.ElseExpression {
	elseExp := &ast.ElseExpression{
		Token:       p.cur_token,
		Consequence: nil,
		NextIfExp:   nil,
	}

	if p.expectPeek(token.IF) {
		ifExp, ok := p.parseIfExpression().(*ast.IfExpression)
		if !ok {
			p.statementErrorf("else if 语句语法错误")
		}
		elseExp.NextIfExp = ifExp
	} else if p.expectPeek(token.LBRACE) {
		elseExp.Consequence = p.parseBlockStatement()
	} else {
		p.statementErrorf("else 语句语法错误")
	}

	return elseExp
}

func (p *Parser) parseFunctionLiteral() ast.Expression {
	funcExp := &ast.FunctionLiteral{
		Token:      p.cur_token,
		Parameters: []*ast.IdentLiteral{},
	}

	if p.expectPeek(token.IDENT) {
		funcExp.Name = &ast.IdentLiteral{
			Token: p.cur_token,
			Value: p.cur_token.Literal,
		}
	} else {
		p.statementErrorf("函数表达式语法错误，缺少函数名")
		return nil
	}

	if p.expectPeek(token.LPAREN) {
		funcExp.Parameters = p.parseFunctionParameters()
	} else {
		p.statementErrorf("函数表达式语法错误，形参列表缺少左括号 '('")
		return nil
	}

	if p.expectPeek(token.LBRACE) {
		funcExp.Body = p.parseBlockStatement()
	} else {
		p.statementErrorf("函数表达式语法错误，缺少函数体")
		return nil
	}

	return funcExp
}

func (p *Parser) parseFunctionParameters() []*ast.IdentLiteral {
	var identifiers []*ast.IdentLiteral

	if p.expectPeek(token.RPAREN) {
		return identifiers
	}

	// 语法解析器 cur_tok 向后移动一个位置
	if p.expectPeek(token.IDENT) {
		ident := &ast.IdentLiteral{
			Token: p.cur_token,
			Value: p.cur_token.Literal,
		}
		identifiers = append(identifiers, ident)
	} else {
		p.statementErrorf("函数表达式语法错误，形参 Token 类型错误")
		return nil
	}

	for p.expectPeek(token.COMMA) {
		// 这个是为了指向 ',' 后面的 Token
		p.nextToken()
		ident := &ast.IdentLiteral{
			Token: p.cur_token,
			Value: p.cur_token.Literal,
		}
		identifiers = append(identifiers, ident)
	}

	if p.expectPeek(token.RPAREN) {
		return identifiers
	} else {
		p.statementErrorf("函数表达式语法错误，形参列表缺失右括号 ')'")
		return nil
	}
}

// parseBlockStatement 解析代码块
func (p *Parser) parseBlockStatement() *ast.BlockStatement {
	bs := &ast.BlockStatement{
		Token:      p.cur_token,
		Statements: []ast.Statement{},
	}

	// 保存当前代码块的起始索引，当代码块发生错误，恢复该索引位置，并取出代码块代码，
	// 之所以需要提前保存，是因为在下面遍历过程中，会修改掉 p.base_index，
	// 所以需要提前保留一份指向代码块起始位置的副本
	baseIndex := p.base_index

	p.nextToken()

	for !p.curTokenIs(token.RBRACE) && !p.curTokenIs(token.EOF) {
		stmt := p.parseStatement()
		if stmt != nil {
			bs.Statements = append(bs.Statements, stmt)
		}
		p.nextToken()
	}

	if !p.curTokenIs(token.RBRACE) {
		// 出现错误则恢复代码块的起始位置
		p.base_index = baseIndex
		p.statementErrorf("代码块缺失右花括号 '}'")
	}

	if p.expectPeek(token.SEMICOLON) {
		p.base_index = baseIndex
		p.statementErrorf("代码块不需要以分号结尾")
	}

	return bs
}

// registerPrefix 注册前缀处理方法
func (p *Parser) registerPrefix(tokenType token.TokenType, fn prefixParseFn) {
	p.prefixParseFns[tokenType] = fn
}

// registerInfix 注册中缀处理方法
func (p *Parser) registerInfix(tokenType token.TokenType, fn infixParseFn) {
	p.infixParseFns[tokenType] = fn
}
