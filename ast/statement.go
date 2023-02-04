package ast

import (
	"bytes"
	"woc_lang/token_v2"
)

// Program 是每个 AST 的根节点，每个有效的程序都是一系列位于 Statements 中的语句
type Program struct {
	Statements []Statement
}

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	} else {
		return ""
	}
}

func (p *Program) String() string {
	var out bytes.Buffer

	for _, s := range p.Statements {
		out.WriteString(s.String())
	}

	return out.String()
}

// VarStatement 声明变量表达式
type VarStatement struct {
	Token *token_v2.Token
	Name  *IdentExpression
	Value Expression
}

func (vs *VarStatement) sNode() {}

func (vs *VarStatement) TokenLiteral() string {
	return vs.Token.Literal
}

func (vs *VarStatement) String() string {
	var out bytes.Buffer

	out.WriteString(vs.TokenLiteral() + " ")
	out.WriteString(vs.Name.String())
	out.WriteString(" = ")
	if vs.Value != nil {
		out.WriteString(vs.Value.String())
	}
	out.WriteString(";")

	return out.String()
}
