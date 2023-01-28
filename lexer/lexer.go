package lexer

import (
	"woc_lang/token"
)

// Lexer 词法分析器
// 使用 rune 读取代码中的字符，因为 golang 中默认使用 UTF-8，需要使用 rune 接收字符，不能使用 byte 接收
type Lexer struct {
	code      []rune // 代码本体
	cur_index int    // 词法分析器指针当前位置
	cur_rune  rune   // 词法分析器当前的字符
	pre_index int    // 词法分析器指针预读取位置
	errs      []string
}

func New(input string) *Lexer {
	l := &Lexer{
		code:      []rune(input),
		cur_index: -1,
		pre_index: 0,
		cur_rune:  0,
	}
	// 这里先读取一次是为了初始化 cur_index, pre_index, cur_rune 这三个字段
	l.readRune()
	return l
}

// NextToken 获取下一个 Token，并更新 current_index 以及 current_rune
func (l *Lexer) NextToken() token.Token {
	var tok token.Token

	l.skipWhitespace()

	switch token.TokenType(l.cur_rune) {
	case token.ASSIGN:
		tok = newToken(token.ASSIGN, string(l.cur_rune))
		// 读取下一个字符判断该 Token 是赋值还是判等
		l.readRune()
		if token.TokenType(l.cur_rune) == token.ASSIGN {
			tok = newToken(token.EQ, "==")
		}
	case token.ASTERISK:
		tok = newToken(token.ASTERISK, string(l.cur_rune))
	case token.ADD:
		tok = newToken(token.ADD, string(l.cur_rune))
	case token.MINUS:
		tok = newToken(token.MINUS, string(l.cur_rune))
	case token.SLASH:
		tok = newToken(token.SLASH, string(l.cur_rune))
	case token.SEMICOLON:
		tok = newToken(token.SEMICOLON, string(l.cur_rune))
	case token.LPAREN:
		tok = newToken(token.LPAREN, string(l.cur_rune))
	case token.RPAREN:
		tok = newToken(token.RPAREN, string(l.cur_rune))
	case token.LBRACE:
		tok = newToken(token.LBRACE, string(l.cur_rune))
	case token.RBRACE:
		tok = newToken(token.RBRACE, string(l.cur_rune))
	case token.END_MARK:
		tok = newToken(token.END_MARK, "")
	default:
		tok = l.readToken()
	}

	l.readRune()

	return tok
}

func (l *Lexer) readToken() token.Token {
	var str string

	// 判断第一个字符是数字还是字母
	if isIdentCharacter(l.cur_rune) {
		str = l.readIdentifier()
	} else if isDigit(l.cur_rune) {
		str = l.readNumber()
		return newToken(token.I32, str)
	}

	// 将关键字转换成 Token
	switch str {
	case "var":
		return newToken(token.VAR, str)
	case "func":
		return newToken(token.FUNC, str)
	case "return":
		return newToken(token.RETURN, str)
	default:
		return newToken(token.IDENT, str)
	}
}

// readStr 读取一个标识符
func (l *Lexer) readIdentifier() string {
	i := l.cur_index

	// 如果是标识符，则允许字母，下划线以及数字组合，不允许以数字开头
	for isIdentCharacter(l.cur_rune) &&
		(isIdentCharacter(l.peerRune()) || isDigit(l.peerRune())) &&
		token.TokenType(l.peerRune()) != token.SEMICOLON &&
		token.TokenType(l.peerRune()) != token.END_MARK {
		l.readRune()
	}

	return string(l.code[i : l.cur_index+1])
}

// readNumber 读取一个由数字组成的字符串
// TODO: 后续会支持浮点数，目前只支持整型
func (l *Lexer) readNumber() string {
	i := l.cur_index

	for isDigit(l.cur_rune) && isDigit(l.peerRune()) &&
		token.TokenType(l.peerRune()) != token.SEMICOLON &&
		token.TokenType(l.peerRune()) != token.END_MARK {
		l.readRune()
	}

	return string(l.code[i : l.cur_index+1])
}

// runeRune 读取下一个字符
func (l *Lexer) readRune() {
	if l.pre_index >= len(l.code) {
		// 读到尾部将 ch 设置为 0，这是 ASCII 码中的空字符
		l.cur_rune = 0
	} else {
		l.cur_rune = l.code[l.pre_index]
	}
	// 将预读取下标更新到当前下标
	l.cur_index = l.pre_index
	// 更新预读取下标
	l.pre_index += 1
}

// skipWhitespace 跳过空白字符
func (l *Lexer) skipWhitespace() {
	// 遇到 空白符，制表符，换行符，回车符就跳过
	for isBlank(l.cur_rune) {
		l.readRune()
	}
}

// peerRune 查看下一个字符，不移动指针
func (l *Lexer) peerRune() rune {
	if l.pre_index >= len(l.code) {
		return 0
	} else {
		return l.code[l.pre_index]
	}
}

// newToken 生成一个正确的可解析的 Token
func newToken(tokenType token.TokenType, tokenLiteral string) token.Token {
	return token.Token{
		Type:    tokenType,
		Literal: tokenLiteral,
	}
}

// newIllegalToken 返回一个非法 token
func newIllegalToken() token.Token {
	return token.Token{
		Type:    token.ILLEGAL,
		Literal: "illegal",
	}
}

// isDigit 判断是否是数字
func isDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

// isIdentCharacter 判断是否是字母和规则允许的字符
func isIdentCharacter(r rune) bool {
	return r >= 'a' && r <= 'z' ||
		r >= 'A' && r <= 'Z' ||
		r == '_'
}

// isBlank 判断是否是空白符
// 空白符，制表符，换行符，回车符 都定义为空白
func isBlank(r rune) bool {
	return token.TokenType(r) == token.BLANK ||
		token.TokenType(r) == token.TAB ||
		token.TokenType(r) == token.LF ||
		token.TokenType(r) == token.CR
}
