package parser

import (
	"fmt"
	"strings"
	"woc_lang/ast"
	"woc_lang/lexer"
	"woc_lang/token"
)

// Parser 语法分析器
// 该语法分析器基于普拉特语法分析器原理实现，使用运算符优先级以及左推导
// 将传入的代码解析为 AST，主要的算法实现在 p.parseExpressionStatement 中体现
type Parser struct {
	l          *lexer.Lexer // 词法分析器
	cur_token  token.Token  // 从词法分析器中读取到的当前 Token
	peek_token token.Token  // 从词法分析器中读取到的下一个 Token
	base_index int          // 语句起始索引
	cur_index  int          // 语句结束索引
	program    *ast.Program // AST 的根节点
	errors     []string     // 收集语法分析过程中出现的错误

	// 用于收集对应的前缀和中缀语法对应的解析函数
	prefixParseFns map[token.TokenType]prefixParseFn
	infixParseFns  map[token.TokenType]infixParseFn
}

func New(l *lexer.Lexer) *Parser {
	p := &Parser{
		l:              l,
		base_index:     0,
		cur_index:      -1,
		errors:         []string{},
		prefixParseFns: make(map[token.TokenType]prefixParseFn),
		infixParseFns:  make(map[token.TokenType]infixParseFn),
	}
	// 初始化 cur_token 和 peek_token
	p.nextToken()
	p.nextToken()

	// 注册解析各类表达式的方法
	RegisterParseFns(p)
	// 解析生成 AST
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
	for p.cur_token.Type != token.EOF {
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
	case token.VAR:
		return p.parseVarStatement()
	case token.RETURN:
		return p.parseReturnStatement()
	default:
		// 如果当前字符不是关键字，那就默认为表达式
		return p.parseExpressionStatement()
	}
}

// parseVarStatement 解析 var 变量声明语句
func (p *Parser) parseVarStatement() ast.Statement {
	// 语句结束之前一定要更新索引
	defer func() {
		p.base_index = p.cur_index
	}()

	stmt := &ast.VarStatement{
		Token: p.cur_token,
	}

	// 判断语法错误
	if !p.expectPeek(token.IDENT) {
		p.errors = append(
			p.errors,
			fmt.Sprintf("解析声明变量语法错误，当前 Token 为 %s，下一个 Token 为 %s",
				p.cur_token.Literal, p.peek_token.Literal),
		)
		return nil
	}

	stmt.Name = ast.IdentLiteral{
		Token: p.cur_token,
		Value: p.cur_token.Literal,
	}

	if !p.expectPeek(token.ASSIGN) {
		p.errors = append(
			p.errors,
			fmt.Sprintf("解析声明变量语法错误，当前 Token 为 %s，下一个 Token 为 %s",
				p.cur_token.Literal, p.peek_token.Literal),
		)
		return nil
	}

	// 移动到下一个 token 位置，并解析
	p.nextToken()
	val := p.parseExpression(LEVEL_0)

	if val == nil {
		p.statementErrorf("解析声明变量语法错误，变量 '%s' 没有进行赋值", stmt.Name.String())
		return nil
	} else {
		stmt.Value = val
	}

	// 检查语句结尾是否符合规则
	if !p.checkStmtEnd() {
		return nil
	}

	return stmt
}

// parseReturnStatement 解析 return 返回声明语句
func (p *Parser) parseReturnStatement() ast.Statement {
	// 语句结束之前一定要更新索引
	defer func() {
		p.base_index = p.cur_index
	}()

	stmt := &ast.ReturnStatement{
		Token: p.cur_token,
	}

	// 移动到下一个 token 位置，并解析
	p.nextToken()
	stmt.ReturnValue = p.parseExpression(LEVEL_0)

	// 检查语句结尾是否符合规则
	if !p.checkStmtEnd() {
		return nil
	}

	return stmt
}

// parseExpressionStatement 解析表达式声明语句
// 表达式解析复杂的一批，实际的解析过程由 p.parseExpression 方法完成
func (p *Parser) parseExpressionStatement() ast.Statement {
	// 语句结束之前一定要更新索引
	defer func() {
		p.base_index = p.cur_index
	}()

	// 这就开始构建表达式节点
	stmt := &ast.ExpressionStatement{
		Token: p.cur_token,
	}
	// 优先给初始表达式节点最低的优先级，以便后续添加表达式
	stmt.Expression = p.parseExpression(LEVEL_0)

	if stmt.Token.Type == token.IF || stmt.Token.Type == token.FUNC {
		return stmt
	} else if !p.checkStmtEnd() {
		return nil
	} else {
		return stmt
	}
}

// parseExpression 表达式解析
// 这个方法是语法解析器，乃至整个普拉特语法解析器的重中之重，表达式解析中，需要使用运算符优先级来进行辅助，
// priority 优先级用于表示右关联性，优先级越高，则右关联性越强，
// 接地气点说，这个值越大，越能粘住右边的表达式并组合成一个新的表达式，比如: 1 + 2 + 3，
// 第一个 + 的优先级要大于数值字面量 2 的优先级，所以将 1 + 2 组成了一个新的表达式 (1 + 2)，
// 然后第二个 + 的优先级大于数值字面量 3，则 (1 + 2) 与 3 通过第二个 + 组合成为((1 + 2) + 3)，
// 右关联性使解析过程中，让右边的 token 尽可能想当前的 token 靠，也就是左推导另类实现，
// 之所以使用左推导，是为了避免在使用右推导时出现的符号变换，
// 举个栗子: x - y - z 使用右推导变成 (x - (y + z))，而使用左推导就是 ((x - y) - z)，
// 这就可以避免在代码解析完成后，语法没问题，但是语义出现了问题
func (p *Parser) parseExpression(priority int) ast.Expression {
	// 分号不做解析
	if p.cur_token.Type == token.SEMICOLON {
		return nil
	}

	// 获取当前 token 的解析方法
	prefix, ok := p.prefixParseFns[p.cur_token.Type]
	if !ok {
		p.noParseFnError(p.cur_token)
		return nil
	}
	leftExp := prefix()

	// 判断是否需要解析中缀表达式
	// precedence < p.peekPrecedence 这一步很关键，举个例子: 5 + 5
	// 第一个 '5' 进来的时候传得是 LOWEST 优先级，也就是最低优先级，
	// 接着 peekToken 变成了 '+'，通过 peekPrecedence() 获得下个一 Token 的优先级，
	// 并与传入的 precedence 相比较，进而判断是否需要进行右关联，
	// '+' 的优先级明显大于 LOWEST 优先级，所以进入了这个 if 分支
	for !p.peekTokenIs(token.SEMICOLON) && priority < p.peekPriority() {
		infix, ok := p.infixParseFns[p.peek_token.Type]
		if !ok {
			return leftExp
		}
		p.nextToken()
		leftExp = infix(leftExp)
	}

	return leftExp
}

func (p *Parser) nextToken() {
	p.cur_token = p.peek_token
	p.peek_token = p.l.NextToken()
	p.cur_index++
}

// curTokenIs 判断当前 Token 是否是指定的 TokenType
func (p *Parser) curTokenIs(t token.TokenType) bool {
	return p.cur_token.Type == t
}

// peekTokenIs 判断下一个 Token 是否是指定的 TokenType
func (p *Parser) peekTokenIs(t token.TokenType) bool {
	return p.peek_token.Type == t
}

// expectPeek 判断下一个 Token 是否为预期的 TokenType
// true - 移动当前 curToken 到下一位
// false - 将错误保存起来
func (p *Parser) expectPeek(t token.TokenType) bool {
	if p.peekTokenIs(t) {
		p.nextToken()
		return true
	} else {
		return false
	}
}

// noParseFnError 找不到解析方法错误
func (p *Parser) noParseFnError(token token.Token) {
	msg := fmt.Sprintf("没有注册 (%s) 的解析方法，需要添加该类型 (TokenType: %d) 的解析方法",
		token.Literal, token.Type)
	p.errors = append(p.errors, msg)
}

// checkStmtEnd 检查语句末尾是否符合规则
func (p *Parser) checkStmtEnd() bool {
	// 按照语句的规则，解析完之后就应该只剩分号(;)
	if p.expectPeek(token.SEMICOLON) {
		return true
	} else {
		p.statementErrorf("语句结束错误，没有分号 ';'")
		return false
	}
}

// endOfStatementError 收集语句结束错误
func (p *Parser) statementErrorf(msgFormat string, args ...any) {
	// 获取一组 token 的字面量
	literals, err := p.l.GetTokensLiteral(p.base_index, p.cur_index)
	if err != nil {
		msg := fmt.Sprintf("Parser 语法分析器错误: %v", err)
		p.errors = append(p.errors, msg)
		return
	}

	// 将字面量组合成语句
	stmt := strings.Join(literals, " ")

	// 格式化错误信息
	msg := fmt.Sprintf(msgFormat, args...)

	// 收集错误
	errMsg := fmt.Sprintf("错误语句: '%s'，错误信息: %s", stmt, msg)
	p.errors = append(p.errors, errMsg)
}
