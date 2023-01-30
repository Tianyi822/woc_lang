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
	code       []rune           // 需要解析的代码
	base_index int              // 需解析 Token 的起始位置，每解析完一个 Token，更新 base_index
	tokens     []token_v2.Token // 收集解析出来的Token，按照顺序存储，向外提供函数访问
}

func New(input string) *Lexer {
	l := &Lexer{
		code:       []rune(input),
		base_index: 0,
		tokens:     []token_v2.Token{},
	}
	l.tokenize()
	return l
}

func (l *Lexer) tokenize() {
	state := dfa_state.Initial
	for _, c := range l.code {
		switch state {
		case dfa_state.Initial: // 初始状态，读取第一个字符
			state = l.stateTrans(c)
		}
	}
}

// stateTrans 状态转移处理
// 有限状态机一开始就进入初始状态,这个初始状态其实并不做停留，它马上进入其他状态。
// 某个 Token 解析完毕，也进入初始状态，在这里把 Token 记下来，然后建立一个新的 Token
func (l *Lexer) stateTrans(ch rune) dfa_state.DfaState {
	return dfa_state.Initial
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
