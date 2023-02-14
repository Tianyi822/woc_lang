package ast

// Node 是 AST 节点，也是上下文无关语法中的终结符
type Node interface {
	// TokenLiteral 凡是实现这个 Node 接口，都必须提供这个方法
	// 该方法返回与其关联的词法单元的字面量，也就是 Token 的 Literal
	TokenLiteral() string
	// String 用于打印 AST 的各个节点
	String() string
}

// Statement 用于解析声明语句
type Statement interface {
	Node
	// 凡是实现 Statement.sNode 接口的都是声明语句
	// 这个语句不做实现，只是为了表示实现的是什么接口（PS：Golang 让我讨厌的语法之一）
	sNode()
}

// Expression 用于解析表达式语句
type Expression interface {
	Node
	eNode()
}
