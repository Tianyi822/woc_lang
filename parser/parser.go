package parser

import (
	"fmt"
	"woc_lang/ast"
	"woc_lang/lexer_v2"
	"woc_lang/token_v2"
)

// Parser 语法分析器
type Parser struct {
	l          *lexer_v2.Lexer // 词法分析器
	cur_token  token_v2.Token  // 从词法分析器中读取到的当前 Token
	peek_token token_v2.Token  // 从词法分析器中读取到的下一个 Token
	program    *ast.Program    // AST 的根节点
	errors     []string        // 收集语法分析过程中出现的错误
}

func New(l *lexer_v2.Lexer) *Parser {
	p := &Parser{
		l:          l,
		cur_token:  l.NextToken(),
		peek_token: l.NextToken(),
		errors:     []string{},
	}

	p.parseProgram()

	return p
}

func (p *Parser) Errors() []string {
	return p.errors
}

// ParseProgram 这里是语法分析器的入口
func (p *Parser) parseProgram() {
	// 构建 AST
	program := &ast.Program{
		Statements: []ast.Statement{},
	}

	// 开始遍历词法单元
	for p.cur_token.Type != token_v2.EOF {
		// 解析一条语句
		stmt := p.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}

		p.nextToken()
	}

	// 保存 AST
	p.program = program
}

// parseStatement 解析语句
func (p *Parser) parseStatement() ast.Statement {
	switch p.cur_token.Type {
	case token_v2.VAR:
		return p.parseVarStatement()
	case token_v2.RETURN:
		return p.parseReturnStatement()
	default:
		// TODO: 保留选择
		return nil
	}
}

func (p *Parser) parseVarStatement() ast.Statement {
	stmt := &ast.VarStatement{
		Token: p.cur_token,
	}

	// 判断语法错误
	if !p.expectPeek(token_v2.IDENT) {
		p.errors = append(
			p.errors,
			fmt.Sprintf("解析声明变量语法错误，当前 Token 为 %s，下一个 Token 为 %s",
				p.cur_token.Literal, p.peek_token.Literal),
		)
		return nil
	}

	stmt.Name = ast.IdentExpression{
		Token: p.cur_token,
		Value: p.cur_token.Literal,
	}

	if !p.expectPeek(token_v2.ASSIGN) {
		p.errors = append(
			p.errors,
			fmt.Sprintf("解析声明变量语法错误，当前 Token 为 %s，下一个 Token 为 %s",
				p.cur_token.Literal, p.peek_token.Literal),
		)
		return nil
	}

	// TODO: 等号右边的表达式暂时不处理，后续添加
	for !p.curTokenIs(token_v2.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseReturnStatement() ast.Statement {
	stmt := &ast.ReturnStatement{
		Token: p.cur_token,
	}

	// TODO: 等号右边的表达式暂时不处理，后续添加
	for !p.curTokenIs(token_v2.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}

func (p *Parser) nextToken() {
	p.cur_token = p.peek_token
	p.peek_token = p.l.NextToken()
}

// curTokenIs 判断当前 Token 是否是指定的 TokenType
func (p *Parser) curTokenIs(t token_v2.TokenType) bool {
	return p.cur_token.Type == t
}

// peekTokenIs 判断下一个 Token 是否是指定的 TokenType
func (p *Parser) peekTokenIs(t token_v2.TokenType) bool {
	return p.peek_token.Type == t
}

// expectPeek 判断下一个 Token 是否为预期的 TokenType
// true - 移动当前 curToken 到下一位
// false - 将错误保存起来
func (p *Parser) expectPeek(t token_v2.TokenType) bool {
	if p.peekTokenIs(t) {
		p.nextToken()
		return true
	} else {
		return false
	}
}
