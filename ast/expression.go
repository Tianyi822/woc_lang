package ast

import (
	"bytes"
	"strings"
	"woc_lang/token"
)

// IdentLiteral 标识符节点，例如 `var x = 5;` 中的 x
// 标识符之所以定义为表达式，是因为当一个值绑定到 `x` 上后，
// x 就指向了这个值，举个栗子：var other_ident = x; 这个语句中，x 就作为表达式
// 将 x 指向的 5 赋值给了 other_ident
type IdentLiteral struct {
	Token token.Token
	Value string
}

func (ie *IdentLiteral) eNode() {}

func (ie *IdentLiteral) TokenLiteral() string {
	return ie.Token.Literal
}

func (ie *IdentLiteral) String() string {
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

// FunctionLiteral 函数字面量
type FunctionLiteral struct {
	Token      token.Token
	Name       *IdentLiteral
	Parameters []*IdentLiteral
	Body       *BlockStatement
}

func (fl *FunctionLiteral) eNode() {}

func (fl *FunctionLiteral) TokenLiteral() string {
	return fl.Token.Literal
}

func (fl *FunctionLiteral) String() string {
	var out bytes.Buffer

	var params []string
	for _, p := range fl.Parameters {
		params = append(params, p.String())
	}

	out.WriteString(fl.TokenLiteral())
	out.WriteString(" ")
	out.WriteString(fl.Name.String())
	out.WriteString("(")
	out.WriteString(strings.Join(params, ", "))
	out.WriteString(") ")
	out.WriteString(fl.Body.String())

	return out.String()
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

type IfExpression struct {
	Token          token.Token
	Condition      Expression
	Consequence    *BlockStatement
	ElseExpression *ElseExpression
}

func (ife *IfExpression) eNode() {}

func (ife *IfExpression) TokenLiteral() string {
	return ife.Token.Literal
}

func (ife *IfExpression) String() string {
	var out bytes.Buffer

	out.WriteString("if ")
	out.WriteString(ife.Condition.String())
	out.WriteString(" ")
	out.WriteString(ife.Consequence.String())

	if ife.ElseExpression != nil {
		out.WriteString(" ")
		out.WriteString(ife.ElseExpression.String())
	}

	return out.String()
}

type ElseExpression struct {
	Token       token.Token
	Consequence *BlockStatement
	NextIfExp   *IfExpression
}

func (ee *ElseExpression) eNode() {}

func (ee *ElseExpression) TokenLiteral() string {
	return ee.Token.Literal
}

func (ee *ElseExpression) String() string {
	var out bytes.Buffer

	out.WriteString("else ")
	if ee.NextIfExp != nil {
		out.WriteString(ee.NextIfExp.String())
	} else {
		out.WriteString(" ")
		out.WriteString(ee.Consequence.String())
	}

	return out.String()
}
