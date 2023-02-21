package ast

import (
	"bytes"
	"woc_lang/token"
)

// IdentExpression 标识符节点，例如 `var x = 5;` 中的 x
// 标识符之所以定义为表达式，是因为当一个值绑定到 `x` 上后，
// x 就指向了这个值，举个栗子：var other_ident = x; 这个语句中，x 就作为表达式
// 将 x 指向的 5 赋值给了 other_ident
type IdentExpression struct {
	Token token.Token
	Value string
}

func (ie *IdentExpression) eNode() {}

func (ie *IdentExpression) TokenLiteral() string {
	return ie.Token.Literal
}

func (ie *IdentExpression) String() string {
	return ie.Value
}

// IntegerLiteral 整型字面量
type IntegerLiteral struct {
	Token token.Token
	Value int64
}

func (il *IntegerLiteral) eNode() {}

func (il *IntegerLiteral) TokenLiteral() string {
	return il.Token.Literal
}

func (il *IntegerLiteral) String() string {
	return il.Token.Literal
}

// BooleanLiteral 布尔类型字面量
type BooleanLiteral struct {
	Token token.Token
	Value bool
}

func (bl *BooleanLiteral) eNode() {}

func (bl *BooleanLiteral) TokenLiteral() string {
	return bl.Token.Literal
}

func (bl *BooleanLiteral) String() string {
	return bl.Token.Literal
}

// PrefixExpression 前缀表达式
type PrefixExpression struct {
	Token    token.Token
	Operator string
	Right    Expression
}

func (pe *PrefixExpression) eNode() {}

func (pe *PrefixExpression) TokenLiteral() string {
	return pe.Token.Literal
}

func (pe *PrefixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(pe.Operator)
	out.WriteString(pe.Right.String())
	out.WriteString(")")

	return out.String()
}

// InfixExpression 中缀表达式
type InfixExpression struct {
	Token    token.Token
	LeftExp  Expression
	Operator string
	RightExp Expression
}

func (ine *InfixExpression) eNode() {}

func (ine *InfixExpression) TokenLiteral() string {
	return ine.Token.Literal
}

func (ine *InfixExpression) String() string {
	var out bytes.Buffer

	out.WriteString("(")
	out.WriteString(ine.LeftExp.String())
	out.WriteString(" " + ine.Operator + " ")
	out.WriteString(ine.RightExp.String())
	out.WriteString(")")

	return out.String()
}
