package lexer

import (
	"fmt"
	"strings"
	"woc_lang/lexer/dfa_state"
	"woc_lang/token"
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
	tokens       []token.Token      // 收集解析出来的Token，按照顺序存储，向外提供函数访问
	cur_state    dfa_state.DfaState // 用于记录分析器当前的状态
	error_tokens []token.ErrToken   // 错误词法集合
	can_be_ident bool               // 判断当前 token 是否可以为 Ident(标识符)
}

func New(input string) *Lexer {
	l := &Lexer{
		code:         append([]rune(input), '\n'),
		reader_index: -1,
		base_index:   0,
		tokens:       []token.Token{},
		cur_state:    dfa_state.Initial,
		error_tokens: []token.ErrToken{},
		can_be_ident: false,
	}
	l.tokenize()
	return l
}

func (l *Lexer) Errors() []token.ErrToken {
	return l.error_tokens
}

func (l *Lexer) NextToken() token.Token {
	if l.reader_index+1 >= len(l.tokens) {
		return token.Token{Type: token.EOF}
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

		//  ==================== 符号解析 ====================
		case dfa_state.Assign_State: // =
			// 判断当前字符是不是 =
			// 如果是，则修改状态为 EQ_State，表示该 Token 是两个 = 组成的判断相等的 Token
			// 如果不是，则表示这只是一个赋值 Token
			if c == '=' {
				l.cur_state = dfa_state.Eq_State
			} else {
				l.dealToken(i, c, token.ASSIGN)
			}
		case dfa_state.Eq_State: // ==
			l.dealToken(i, c, token.EQ)

		case dfa_state.Minus_State: // -
			if c == '>' {
				l.cur_state = dfa_state.Arrow_State
			} else {
				l.dealToken(i, c, token.MINUS)
			}
		case dfa_state.Arrow_State: // ->
			l.dealToken(i, c, token.ARROW)

		case dfa_state.Bang_State: // !
			if c == '=' {
				l.cur_state = dfa_state.Neq_State
			} else {
				l.dealToken(i, c, token.BANG)
			}
		case dfa_state.Neq_State: // !=
			l.dealToken(i, c, token.NEQ)

		case dfa_state.Gt_State:
			if c == '=' {
				l.cur_state = dfa_state.Ge_State
			} else if c == '>' {
				l.cur_state = dfa_state.Bit_R_Offset_State
			} else {
				l.dealToken(i, c, token.GT)
			}
		case dfa_state.Ge_State:
			l.dealToken(i, c, token.GE)
		case dfa_state.Bit_R_Offset_State:
			l.dealToken(i, c, token.BIT_R_OFFSET)

		case dfa_state.Lt_State:
			if c == '=' {
				l.cur_state = dfa_state.Le_State
			} else if c == '<' {
				l.cur_state = dfa_state.Bit_L_Offset_State
			} else {
				l.dealToken(i, c, token.LT)
			}
		case dfa_state.Le_State:
			l.dealToken(i, c, token.LE)
		case dfa_state.Bit_L_Offset_State:
			l.dealToken(i, c, token.BIT_L_OFFSET)

		case dfa_state.Bit_And_State:
			if c == '&' {
				l.cur_state = dfa_state.And_State
			} else {
				l.dealToken(i, c, token.BIT_AND)
			}
		case dfa_state.And_State:
			l.dealToken(i, c, token.AND)

		case dfa_state.Bit_Or_State:
			if c == '|' {
				l.cur_state = dfa_state.Or_State
			} else {
				l.addToken(i, token.BIT_OR)
			}
		case dfa_state.Or_State:
			l.dealToken(i, c, token.OR)

		//  ==================== 关键字解析 ====================
		//  ==================== func ====================
		case dfa_state.Func_State_1: // func
			if c == 'u' {
				l.cur_state = dfa_state.Func_State_2
			} else if c == 'a' {
				l.cur_state = dfa_state.False_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Func_State_2:
			if c == 'n' {
				l.cur_state = dfa_state.Func_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Func_State_3:
			if c == 'c' {
				l.cur_state = dfa_state.Func_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Func_State:
			l.createTokenOrTransState(i, c, token.FUNC)

		//  ==================== meth ====================
		case dfa_state.Meth_State_1: // meth
			if c == 'e' {
				l.cur_state = dfa_state.Meth_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Meth_State_2:
			if c == 't' {
				l.cur_state = dfa_state.Meth_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Meth_State_3:
			if c == 'h' {
				l.cur_state = dfa_state.Meth_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Meth_State:
			l.createTokenOrTransState(i, c, token.METH)

		//  ==================== var ====================
		case dfa_state.Var_State_1: // var
			if c == 'a' {
				l.cur_state = dfa_state.Var_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Var_State_2:
			if c == 'r' {
				l.cur_state = dfa_state.Var_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Var_State:
			l.createTokenOrTransState(i, c, token.VAR)

		//  ==================== bool ====================
		case dfa_state.Bool_State_1: // bool
			if c == 'o' {
				l.cur_state = dfa_state.Bool_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Bool_State_2:
			if c == 'o' {
				l.cur_state = dfa_state.Bool_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Bool_State_3:
			if c == 'l' {
				l.cur_state = dfa_state.Bool_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Bool_State:
			l.createTokenOrTransState(i, c, token.BOOL)

		//  ==================== true ====================
		case dfa_state.True_State_1: // true
			if c == 'r' {
				l.cur_state = dfa_state.True_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.True_State_2:
			if c == 'u' {
				l.cur_state = dfa_state.True_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.True_State_3:
			if c == 'e' {
				l.cur_state = dfa_state.True_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.True_State:
			l.createTokenOrTransState(i, c, token.TRUE)

		//  ==================== false ====================
		case dfa_state.False_State_2: // false
			if c == 'l' {
				l.cur_state = dfa_state.False_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.False_State_3:
			if c == 's' {
				l.cur_state = dfa_state.False_State_4
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.False_State_4:
			if c == 'e' {
				l.cur_state = dfa_state.False_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.False_State:
			l.createTokenOrTransState(i, c, token.FALSE)

		//  ==================== if ====================
		case dfa_state.If_State_1: // if
			if c == 'f' {
				l.cur_state = dfa_state.If_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.If_State:
			l.createTokenOrTransState(i, c, token.IF)

		//  ==================== else ====================
		case dfa_state.Else_State_1: // else
			if c == 'l' {
				l.cur_state = dfa_state.Else_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Else_State_2:
			if c == 's' {
				l.cur_state = dfa_state.Else_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Else_State_3:
			if c == 'e' {
				l.cur_state = dfa_state.Else_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Else_State:
			l.createTokenOrTransState(i, c, token.ELSE)

		//  ==================== return ====================
		case dfa_state.Return_State_1: // return
			if c == 'e' {
				l.cur_state = dfa_state.Return_State_2
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Return_State_2:
			if c == 't' {
				l.cur_state = dfa_state.Return_State_3
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Return_State_3:
			if c == 'u' {
				l.cur_state = dfa_state.Return_State_4
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Return_State_4:
			if c == 'r' {
				l.cur_state = dfa_state.Return_State_5
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Return_State_5:
			if c == 'n' {
				l.cur_state = dfa_state.Return_State
			} else {
				l.createTokenOrTransState(i, c, token.IDENT)
			}
		case dfa_state.Return_State:
			l.createTokenOrTransState(i, c, token.RETURN)

		//  ==================== 标识符定义 ====================
		//  ==================== ident ====================
		case dfa_state.Ident_State:
			l.createTokenOrTransState(i, c, token.IDENT)

		//  ==================== 数值字面量定义 ====================
		case dfa_state.Num_State: // 822
			if isDigit(c) {
				l.cur_state = dfa_state.Num_State
			} else {
				// 当前状态结束时，需要识别新的状态
				l.dealToken(i, c, token.NUM)
			}

		default:
			// 未定义状态的就是空字符
			continue
		}
	}
	// 扫描结束后，在 Token 集合中添加 EOF Token 表示结束
	l.stateTrans(len(l.code), 0)
}

// createTokenOrTransState 根据传入的参数判断是生成一个新的 Token，还是转移状态
func (l *Lexer) createTokenOrTransState(index int, ch rune, tokenType token.TokenType) {
	if isIdentLetter(ch) {
		l.cur_state = dfa_state.Ident_State
	} else {
		l.dealToken(index, ch, tokenType)
	}
}

// dealToken Token 添加完成后，根据当前读到的新字符，转移到对应状态
func (l *Lexer) dealToken(index int, c rune, tokenType token.TokenType) {
	l.addToken(index, tokenType)
	l.stateTrans(index, c)
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

	if isLetter(ch) { // 字母解析
		// 如果当前标识符是以字母开头，则允许作为标识符
		if l.cur_state == dfa_state.Initial {
			l.can_be_ident = true
		}
		l.letterState(ch)
	} else if isDigit(ch) { // 数字解析
		// 如果当前标识符是以数字开头，则允许作为标识符
		if l.cur_state == dfa_state.Initial {
			l.can_be_ident = false
		}
		l.cur_state = dfa_state.Num_State
	} else if isBlank(ch) {
		l.base_index += 1
	} else {
		// 符号解析
		// 只有当前状态为 Initial 状态，且当前字符为 _ 时，才可以作为标识符
		if l.cur_state == dfa_state.Initial && ch == '_' {
			l.can_be_ident = true
		}
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
	default:
		l.cur_state = dfa_state.Ident_State
	}
}

func (l *Lexer) symbolState(end_index int, ch rune) {
	switch ch {
	case ',':
		// 将解析到的 Token 添加到集合中，并修改分析器状态
		l.addToken(end_index, token.COMMA)

	case '.':
		l.addToken(end_index, token.DOT)

	case ';':
		l.addToken(end_index, token.SEMICOLON)

	case ':':
		l.addToken(end_index, token.COLON)

	case '_':
		l.cur_state = dfa_state.Ident_State

	case '+':
		l.addToken(end_index, token.ADD)

	case '*':
		l.addToken(end_index, token.ASTERISK)

	case '/':
		l.addToken(end_index, token.SLASH)

	case '(':
		l.addToken(end_index, token.LPAREN)

	case ')':
		l.addToken(end_index, token.RPAREN)

	case '[':
		l.addToken(end_index, token.LBRACKET)

	case ']':
		l.addToken(end_index, token.RBRACKET)

	case '{':
		l.addToken(end_index, token.LBRACE)

	case '}':
		l.addToken(end_index, token.RBRACE)

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
func (l *Lexer) addToken(end_index int, tokenType token.TokenType) {
	var tokLiteral string
	// 针对单字符，例如: , . ; : + * / ( ) { } [ ]
	if l.base_index == end_index {
		tokLiteral = string(l.code[l.base_index])
		l.base_index = end_index + 1
	} else {
		// 消除 Token 中的空格
		tokLiteral = strings.TrimSpace(string(l.code[l.base_index:end_index]))
		l.base_index = end_index
	}

	ok := l.checkToken(end_index, tokenType, tokLiteral)
	if ok {
		tok := token.Token{
			Type:    tokenType,
			Literal: tokLiteral,
		}
		l.tokens = append(l.tokens, tok)
	}

	// 当一个 Token 添加到集合后，就需要重置状态
	l.cur_state = dfa_state.Initial
}

// checkToken 检查是否有定义此类型 Token
func (l *Lexer) checkToken(end_index int, tokenType token.TokenType, tokLiteral string) bool {
	ok := true
	// 因为数值字面量和标识符字面量不确定，所以当传递过来的是 Num 和 IDENT 类型，就不需要检查
	if tokenType != token.NUM && tokenType != token.IDENT {
		// 判断关键字表中是否存在
		_, ok = token.TokenMap[tokLiteral]
	}

	// 如果当前的 tokenType 是数值类型，且不可以为标识符时，表示当前 Token 错误
	if !l.can_be_ident && tokenType == token.IDENT {
		msg := fmt.Sprintf("[%d]-(%s) 到 [%d]-(%s) 的标识符格式错误，不允许以数字开头: %s",
			l.base_index+1, string(l.code[l.base_index]), end_index+1, string(l.code[end_index]), tokLiteral)
		errTok := token.ErrToken{
			Type:    token.ILLEGAL,
			Literal: tokLiteral,
			Msg:     msg,
		}
		l.error_tokens = append(l.error_tokens, errTok)
		return ok
	}

	if !ok {
		msg := fmt.Sprintf("[%d]-(%s) 到 [%d]-(%s) 的符号/关键字未定义，请检查代码是否有误",
			l.base_index+1, string(l.code[l.base_index]), end_index+1, string(l.code[end_index]))

		errTok := token.ErrToken{
			Type:    token.ILLEGAL,
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
	l.tokens = append(l.tokens, token.Token{
		Type:    token.EOF,
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

// isIdentLetter 判断该字符是否符合标识符的定义
func isIdentLetter(r rune) bool {
	return r >= 'a' && r <= 'z' ||
		r >= 'A' && r <= 'Z' ||
		r == '_' ||
		r >= '0' && r <= '9'
}

// isBlank 判断是否是空白符
// 空白符，制表符，换行符，回车符 都定义为空白
func isBlank(r rune) bool {
	return r == ' ' || r == '\n' || r == '\r' || r == '\t'
}
