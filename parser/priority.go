package parser

import "woc_lang/token"

const (
	LOW_EST_LEVEL = iota
	LEVEL_1
	LEVEL_2
	LEVEL_3
	PREFIX_LEVEL
)

var prioritiesTable = map[token.TokenType]int{
	token.EQ:    LEVEL_1,
	token.NEQ:   LEVEL_1,
	token.LT:    LEVEL_2,
	token.GT:    LEVEL_2,
	token.LE:    LEVEL_2,
	token.GE:    LEVEL_2,
	token.ADD:   LEVEL_3,
	token.MINUS: PREFIX_LEVEL,
	token.BANG:  PREFIX_LEVEL,
}
