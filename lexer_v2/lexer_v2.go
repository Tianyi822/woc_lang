package lexer_v2

import (
	"woc_lang/lexer_v2/dfa_state"
	"woc_lang/token_v2"
)

// 新版本的词法分析器根据有限自动机概念重新设计：
// S(有限状态集): 定义在 dfa_state 包中
// Σ(有限字母表): 即 a-z A-Z
// σ(转移函数): 在 stateTrans 中实现，其实在这里也可以称为转移逻辑
// s0 ∈ S: 起始状态即 dfa_state.Initial
// SA(接收状态集): 也定义在 dfa_state 包中

// Lexer 第二版词法分析器，相较于第一版本的分析器，添加了状态检查
// 将解析过程规范化，基于 '确定有限自动机' 原理实现，
type Lexer struct {
	code         []rune             // 需要解析的代码
	reader_index int                // NextToken 方法中使用该指针指向当前需要读取的 Token
	base_index   int                // 需解析 Token 的起始位置，每解析完一个 Token，更新 base_index
	tokens       []token_v2.Token   // 收集解析出来的Token，按照顺序存储，向外提供函数访问
	state        dfa_state.DfaState // 用于记录分析器当前的状态
}

func New(input string) *Lexer {
	l := &Lexer{
		code:         []rune(input),
		reader_index: -1,
		base_index:   0,
		tokens:       []token_v2.Token{},
		state:        dfa_state.Initial,
	}
	l.tokenize()
	return l
}

func (l *Lexer) NextToken() token_v2.Token {
	if l.reader_index >= len(l.tokens) {
		return token_v2.Token{Type: token_v2.EOF}
	} else {
		l.reader_index += 1
		return l.tokens[l.reader_index]
	}
}

// tokenize 识别代码中的 token
func (l *Lexer) tokenize() {
	for i, c := range l.code {
		switch l.state {
		case dfa_state.Initial: // 初始状态，读取第一个字符
			l.stateTrans(i, c)
		case dfa_state.Comma_State, // ,
			dfa_state.Dot_State,                            // .
			dfa_state.Semicolon_State,                      // ;
			dfa_state.Colon_State,                          // :
			dfa_state.Underline_State,                      // _
			dfa_state.Lparen_State, dfa_state.Rparen_State, // ()
			dfa_state.Lbrace_State, dfa_state.Rbrace_State, // {}
			dfa_state.Lbracket_State, dfa_state.Rbracket_State: // []
			l.stateTrans(i, c)
		default:
			// 未定义状态的就是空字符
			continue
		}
	}
	// 扫描结束后，在 Token 集合中添加 EOF Token 表示结束
	l.stateTrans(len(l.code), 0)
}

// stateTrans 状态转移处理
// 有限状态机一开始就进入初始状态,这个初始状态其实并不做停留，它马上进入其他状态。
// 某个 Token 解析完毕，也进入初始状态，在这里把 Token 记下来，然后建立一个新的 Token
func (l *Lexer) stateTrans(i int, ch rune) {
	// 这里 +1 是为了从 code 中切出来正确的 Token
	end_index := i + 1
	switch ch {
	case ',':
		// 将解析到的 Token 添加到集合中，并修改分析器状态
		l.state = dfa_state.Comma_State
		l.addToken(token_v2.COMMA, string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	case '.':
		l.state = dfa_state.Dot_State
		l.addToken(token_v2.DOT, string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	case ';':
		l.state = dfa_state.Semicolon_State
		l.addToken(token_v2.SEMICOLON, string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	case ':':
		l.state = dfa_state.Colon_State
		l.addToken(token_v2.COLON, string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	case '_':
		l.state = dfa_state.Underline_State
		l.addToken(token_v2.UNDERLINE, string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	case 0:
		l.state = dfa_state.End
		l.addToken(token_v2.EOF, string(rune(0)))
	}
}

// addToken 将解析出来的 token 添加到集合中
// param tokenType: Token 类型
// param literal: Token 字面量
func (l *Lexer) addToken(tokenType token_v2.TokenType, literal string) {
	tok := token_v2.Token{
		Type:    tokenType,
		Literal: literal,
	}

	l.tokens = append(l.tokens, tok)
}

// isDigit 判断是否是数字
func isDigit(r rune) bool {
	return r >= '0' && r <= '9'
}

// isLetter 判断是否为字母
func isLetter(r rune) bool {
	return r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z'
}

// isBlank 判断是否是空白符
// 空白符，制表符，换行符，回车符 都定义为空白
func isBlank(r rune) bool {
	return r == ' ' || r == '\n' || r == '\r' || r == '\t'
}
