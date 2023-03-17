package ast

import (
	"bytes"
	"woc_lang/token"
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
		out.WriteString(";")
	}

	return out.String()
}

// VarStatement 声明变量表达式
type VarStatement struct {
	Token token.Token
	Name  IdentLiteral
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

	return out.String()
}

// ReturnStatement 声明返回语句
type ReturnStatement struct {
	Token       token.Token
	ReturnValue Expression
}

func (rs *ReturnStatement) sNode() {}

func (rs *ReturnStatement) TokenLiteral() string {
	return rs.Token.Literal
}

func (rs *ReturnStatement) String() string {
	var out bytes.Buffer

	out.WriteString(rs.TokenLiteral() + " ")
	if rs.ReturnValue != nil {
		out.WriteString(rs.ReturnValue.String())
	}

	return out.String()
}

// ExpressionStatement 表达式声明语句
// 实现了 Statement 接口，意味着表达式语句可以添加到 Program 的 Statement 中
// 这也就是为什么已经有了 Expression 接口还要在封装一层
type ExpressionStatement struct {
	Token      token.Token // 该表达式中的第一个词法单元
	Expression Expression
}

func (es *ExpressionStatement) sNode() {}

func (es *ExpressionStatement) TokenLiteral() string {
	return es.Token.Literal
}

func (es *ExpressionStatement) String() string {
	if es.Expression != nil {
		return es.Expression.String()
	}

	return ""
}

type BlockStatement struct {
	Token      token.Token
	Statements []Statement
}

func (bs *BlockStatement) sNode() {}

func (bs *BlockStatement) TokenLiteral() string {
	return bs.Token.Literal
}

func (bs *BlockStatement) String() string {
	var out bytes.Buffer

	for _, s := range bs.Statements {
		out.WriteString(s.String())
		out.WriteString(";")
	}

	return out.String()
}
