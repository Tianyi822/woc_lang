package lexer_v2

import (
	"fmt"
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
	code         []rune              // 需要解析的代码
	reader_index int                 // NextToken 方法中使用该指针指向当前需要读取的 Token
	base_index   int                 // 需解析 Token 的起始位置，每解析完一个 Token，更新 base_index
	tokens       []token_v2.Token    // 收集解析出来的Token，按照顺序存储，向外提供函数访问
	cur_state    dfa_state.DfaState  // 用于记录分析器当前的状态
	error_tokens []token_v2.ErrToken // 错误词法集合
}

func New(input string) *Lexer {
	l := &Lexer{
		code:         append([]rune(input), '\n'),
		reader_index: -1,
		base_index:   0,
		tokens:       []token_v2.Token{},
		cur_state:    dfa_state.Initial,
		error_tokens: []token_v2.ErrToken{},
	}
	l.tokenize()
	return l
}

func (l *Lexer) Errors() []token_v2.ErrToken {
	return l.error_tokens
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
		switch l.cur_state {
		case dfa_state.Initial: // 初始状态，读取第一个字符
			l.stateTrans(i, c)

		case dfa_state.Comma_State, // ,
			dfa_state.Dot_State,                            // .
			dfa_state.Semicolon_State,                      // ;
			dfa_state.Colon_State,                          // :
			dfa_state.Underline_State,                      // _
			dfa_state.Add_State,                            // +
			dfa_state.Asterisk_State,                       // *
			dfa_state.Slash_State,                          // /
			dfa_state.Lparen_State, dfa_state.Rparen_State, // ()
			dfa_state.Lbrace_State, dfa_state.Rbrace_State, // {}
			dfa_state.Lbracket_State, dfa_state.Rbracket_State: // []
			l.stateTrans(i, c)

		case dfa_state.Assign_State: // =
			// 判断当前字符是不是 =
			// 如果是，则修改状态为 EQ_State，表示该 Token 是两个 = 组成的判断相等的 Token
			// 如果不是，则表示这只是一个赋值 Token
			if c == '=' {
				l.cur_state = dfa_state.Eq_State
			} else {
				l.addToken(i, token_v2.ASSIGN)
			}
		case dfa_state.Eq_State: // ==
			l.addToken(i, token_v2.EQ)

		case dfa_state.Minus_State: // -
			if c == '>' {
				l.cur_state = dfa_state.Arrow_State
			} else {
				l.addToken(i, token_v2.MINUS)
			}
		case dfa_state.Arrow_State: // ->
			l.addToken(i, token_v2.ARROW)

		case dfa_state.Bang_State: // !
			if c == '=' {
				l.cur_state = dfa_state.Neq_State
			} else {
				l.addToken(i, token_v2.BANG)
			}
		case dfa_state.Neq_State: // !=
			l.addToken(i, token_v2.NEQ)

		case dfa_state.Gt_State:
			if c == '=' {
				l.cur_state = dfa_state.Ge_State
			} else if c == '>' {
				l.cur_state = dfa_state.Bit_R_Offset_State
			} else {
				l.addToken(i, token_v2.GT)
			}
		case dfa_state.Ge_State:
			l.addToken(i, token_v2.GE)
		case dfa_state.Bit_R_Offset_State:
			l.addToken(i, token_v2.BIT_R_OFFSET)

		case dfa_state.Lt_State:
			if c == '=' {
				l.cur_state = dfa_state.Le_State
			} else if c == '<' {
				l.cur_state = dfa_state.Bit_L_Offset_State
			} else {
				l.addToken(i, token_v2.LT)
			}
		case dfa_state.Le_State:
			l.addToken(i, token_v2.LE)
		case dfa_state.Bit_L_Offset_State:
			l.addToken(i, token_v2.BIT_L_OFFSET)

		case dfa_state.Bit_And_State:
			if c == '&' {
				l.cur_state = dfa_state.And_State
			} else {
				l.addToken(i, token_v2.BIT_AND)
			}
		case dfa_state.And_State:
			l.addToken(i, token_v2.AND)

		case dfa_state.Bit_Or_State:
			if c == '|' {
				l.cur_state = dfa_state.Or_State
			} else {
				l.addToken(i, token_v2.BIT_OR)
			}
		case dfa_state.Or_State:
			l.addToken(i, token_v2.OR)

		case dfa_state.Func_State_1: // func
			if c == 'u' {
				l.cur_state = dfa_state.Func_State_2
			} else if c == 'a' {
				l.cur_state = dfa_state.False_State_2
			}
		case dfa_state.Func_State_2:
			if c == 'n' {
				l.cur_state = dfa_state.Func_State_3
			}
		case dfa_state.Func_State_3:
			if c == 'c' {
				l.cur_state = dfa_state.Func_State
			}
		case dfa_state.Func_State:
			l.addToken(i, token_v2.FUNC)

		case dfa_state.Meth_State_1: // meth
			if c == 'e' {
				l.cur_state = dfa_state.Meth_State_2
			}
		case dfa_state.Meth_State_2:
			if c == 't' {
				l.cur_state = dfa_state.Meth_State_3
			}
		case dfa_state.Meth_State_3:
			if c == 'h' {
				l.cur_state = dfa_state.Meth_State
			}
		case dfa_state.Meth_State:
			l.addToken(i, token_v2.METH)

		case dfa_state.Var_State_1: // var
			if c == 'a' {
				l.cur_state = dfa_state.Var_State_2
			}
		case dfa_state.Var_State_2:
			if c == 'r' {
				l.cur_state = dfa_state.Var_State
			}
		case dfa_state.Var_State:
			l.addToken(i, token_v2.VAR)

		case dfa_state.Bool_State_1: // bool
			if c == 'o' {
				l.cur_state = dfa_state.Bool_State_2
			}
		case dfa_state.Bool_State_2:
			if c == 'o' {
				l.cur_state = dfa_state.Bool_State_3
			}
		case dfa_state.Bool_State_3:
			if c == 'l' {
				l.cur_state = dfa_state.Bool_State
			}
		case dfa_state.Bool_State:
			l.addToken(i, token_v2.BOOL)

		case dfa_state.True_State_1: // true
			if c == 'r' {
				l.cur_state = dfa_state.True_State_2
			}
		case dfa_state.True_State_2:
			if c == 'u' {
				l.cur_state = dfa_state.True_State_3
			}
		case dfa_state.True_State_3:
			if c == 'e' {
				l.cur_state = dfa_state.True_State
			}
		case dfa_state.True_State:
			l.addToken(i, token_v2.TRUE)

		case dfa_state.False_State_2: // false
			if c == 'l' {
				l.cur_state = dfa_state.False_State_3
			}
		case dfa_state.False_State_3:
			if c == 's' {
				l.cur_state = dfa_state.False_State_4
			}
		case dfa_state.False_State_4:
			if c == 'e' {
				l.cur_state = dfa_state.False_State
			}
		case dfa_state.False_State:
			l.addToken(i, token_v2.FALSE)

		case dfa_state.If_State_1: // if
			if c == 'f' {
				l.cur_state = dfa_state.If_State
			} else if c == 'n' {
				l.cur_state = dfa_state.Int32_State_2
			}
		case dfa_state.If_State:
			l.addToken(i, token_v2.IF)

		case dfa_state.Else_State_1: // else
			if c == 'l' {
				l.cur_state = dfa_state.Else_State_2
			}
		case dfa_state.Else_State_2:
			if c == 's' {
				l.cur_state = dfa_state.Else_State_3
			}
		case dfa_state.Else_State_3:
			if c == 'e' {
				l.cur_state = dfa_state.Else_State
			}
		case dfa_state.Else_State:
			l.addToken(i, token_v2.ELSE)

		case dfa_state.Num_State: // 822
			if isDigit(c) {
				l.cur_state = dfa_state.Num_State
			} else {
				l.addToken(i, token_v2.NUM)
			}

		case dfa_state.Int32_State_2:
			if c == 't' {
				l.cur_state = dfa_state.Int32_State_3
			}
		case dfa_state.Int32_State_3:
			if c == '3' {
				l.cur_state = dfa_state.Int32_State_4
			}
		case dfa_state.Int32_State_4:
			if c == '2' {
				l.cur_state = dfa_state.Int32_State
			}
		case dfa_state.Int32_State:
			l.addToken(i, token_v2.INT32)

		case dfa_state.Return_State_1: // return
			if c == 'e' {
				l.cur_state = dfa_state.Return_State_2
			}
		case dfa_state.Return_State_2:
			if c == 't' {
				l.cur_state = dfa_state.Return_State_3
			}
		case dfa_state.Return_State_3:
			if c == 'u' {
				l.cur_state = dfa_state.Return_State_4
			}
		case dfa_state.Return_State_4:
			if c == 'r' {
				l.cur_state = dfa_state.Return_State_5
			}
		case dfa_state.Return_State_5:
			if c == 'n' {
				l.cur_state = dfa_state.Return_State
			}
		case dfa_state.Return_State:
			l.addToken(i, token_v2.RETURN)

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
func (l *Lexer) stateTrans(end_index int, ch rune) {
	// 如果当前字符的 ASCII 码为 0，则表示已经到达末尾，修改状态为结束态
	if ch == 0 {
		l.addEOFToken()
		return
	}

	if isLetter(ch) {
		l.letterState(ch)
	} else if isDigit(ch) {
		l.cur_state = dfa_state.Num_State
	} else {
		l.symbolState(end_index, ch)
	}
}

func (l *Lexer) letterState(ch rune) {
	switch ch {
	case 'f':
		l.cur_state = dfa_state.Func_State_1
	case 'v':
		l.cur_state = dfa_state.Var_State_1
	case 'b':
		l.cur_state = dfa_state.Bool_State_1
	case 't':
		l.cur_state = dfa_state.True_State_1
	case 'i':
		l.cur_state = dfa_state.If_State_1
	case 'e':
		l.cur_state = dfa_state.Else_State_1
	case 'm':
		l.cur_state = dfa_state.Meth_State_1
	case 'r':
		l.cur_state = dfa_state.Return_State_1
	}
}

func (l *Lexer) symbolState(end_index int, ch rune) {
	switch ch {
	case ',':
		// 将解析到的 Token 添加到集合中，并修改分析器状态
		l.cur_state = dfa_state.Comma_State
		l.addToken(end_index, token_v2.COMMA)

	case '.':
		l.cur_state = dfa_state.Dot_State
		l.addToken(end_index, token_v2.DOT)

	case ';':
		l.cur_state = dfa_state.Semicolon_State
		l.addToken(end_index, token_v2.SEMICOLON)

	case ':':
		l.cur_state = dfa_state.Colon_State
		l.addToken(end_index, token_v2.COLON)

	case '_':
		l.cur_state = dfa_state.Underline_State
		l.addToken(end_index, token_v2.UNDERLINE)

	case '+':
		l.cur_state = dfa_state.Add_State
		l.addToken(end_index, token_v2.ADD)

	case '*':
		l.cur_state = dfa_state.Asterisk_State
		l.addToken(end_index, token_v2.ASTERISK)

	case '/':
		l.cur_state = dfa_state.Slash_State
		l.addToken(end_index, token_v2.SLASH)

	case '(':
		l.cur_state = dfa_state.Lparen_State
		l.addToken(end_index, token_v2.LPAREN)

	case ')':
		l.cur_state = dfa_state.Rparen_State
		l.addToken(end_index, token_v2.RPAREN)

	case '[':
		l.cur_state = dfa_state.Lbracket_State
		l.addToken(end_index, token_v2.LBRACKET)

	case ']':
		l.cur_state = dfa_state.Rbracket_State
		l.addToken(end_index, token_v2.RBRACKET)

	case '{':
		l.cur_state = dfa_state.Lbrace_State
		l.addToken(end_index, token_v2.LBRACE)

	case '}':
		l.cur_state = dfa_state.Rbrace_State
		l.addToken(end_index, token_v2.RBRACE)

	case '-':
		l.cur_state = dfa_state.Minus_State

	case '=':
		l.cur_state = dfa_state.Assign_State

	case '!':
		l.cur_state = dfa_state.Bang_State

	case '>':
		l.cur_state = dfa_state.Gt_State

	case '<':
		l.cur_state = dfa_state.Lt_State

	case '&':
		l.cur_state = dfa_state.Bit_And_State

	case '|':
		l.cur_state = dfa_state.Bit_Or_State
	}
}

// addToken 将解析出来的 token 添加到集合中
// param end_index: Token 字符结束位置
// param tokenType: Token 类型
func (l *Lexer) addToken(end_index int, tokenType token_v2.TokenType) {
	// 消除 Token 中的空格，举个栗子：
	// 有一个字符串 (= ==), 要从中解析出 (=)，按照当前的代码逻辑，end_index 指向第一个空格
	// addToken 需要将 (= ) 修正为 (=)，这样就可以放心处理空格问题，无需在前面的逻辑中添加复杂代码
	// 至于为什么不用 golang 提供的内置函数，是为了减少性能消耗，因为将 rune 数组转换为字符串会有一定的损耗
	var realToken []rune
	for i := l.base_index; i < end_index+1; i++ {
		if !isBlank(l.code[i]) {
			realToken = append(realToken, l.code[i])
		}
	}

	tokLiteral := string(realToken)
	l.checkToken(tokenType, tokLiteral)
	l.base_index = end_index + 1

	// 当一个 Token 添加到集合后，就需要重置状态
	l.cur_state = dfa_state.Initial
}

// checkToken 检查是否有定义此类型 Token
func (l *Lexer) checkToken(tokenType token_v2.TokenType, tokLiteral string) bool {
	ok := true
	// 因为数值字面量不确定，所以当传递过来的是 Num 类型，就不需要检查
	if tokenType != token_v2.NUM {
		// 判断关键字表中是否存在
		_, ok = token_v2.TokenMap[tokLiteral]
	}

	if ok || tokenType == token_v2.NUM {
		tok := token_v2.Token{
			Type:    tokenType,
			Literal: tokLiteral,
		}
		l.tokens = append(l.tokens, tok)
	} else {
		msg := fmt.Sprintf("该符号/关键字未定义，请检查代码是否有误")

		errTok := token_v2.ErrToken{
			Type:    token_v2.ILLEGAL,
			Literal: tokLiteral,
			Msg:     msg,
		}

		l.error_tokens = append(l.error_tokens, errTok)
	}
	return ok
}

// addEOFToken 添加结束 Token
func (l *Lexer) addEOFToken() {
	l.cur_state = dfa_state.End
	l.tokens = append(l.tokens, token_v2.Token{
		Type:    token_v2.EOF,
		Literal: "",
	})
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
