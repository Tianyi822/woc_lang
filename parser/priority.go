package parser

import "woc_lang/token"

const (
	LOW_EST_LEVEL = iota
	LEVEL_1
	LEVEL_2
	LEVEL_3
	LEVEL_4
	PREFIX_LEVEL
)

var infixOpPriorityMap = map[token.TokenType]int{
	token.EQ:       LEVEL_1,
	token.NEQ:      LEVEL_1,
	token.LT:       LEVEL_2,
	token.GT:       LEVEL_2,
	token.LE:       LEVEL_2,
	token.GE:       LEVEL_2,
	token.ADD:      LEVEL_3,
	token.MINUS:    LEVEL_3,
	token.ASTERISK: LEVEL_4,
	token.SLASH:    LEVEL_4,
}

// peekPriority 检查下一个 Token 的优先级
func (p *Parser) peekPriority() int {
	if p, ok := infixOpPriorityMap[p.peek_token.Type]; ok {
		return p
	}

	return LOW_EST_LEVEL
}

func (p *Parser) curPriority() int {
	if p, ok := infixOpPriorityMap[p.cur_token.Type]; ok {
		return p
	}

	return LOW_EST_LEVEL
}
