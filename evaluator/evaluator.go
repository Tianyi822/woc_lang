package evaluator

import (
	"woc_lang/ast"
	"woc_lang/object"
)

// 这里为了不用对着三个对象重复构造浪费内存，整个程序中这三个对象各自只有一份实例
var (
	NULL  = &object.NULL{}
	TRUE  = &object.Boolean{Value: true}
	FALSE = &object.Boolean{Value: false}
)

func Eval(node ast.Node) object.Object {
	switch node := node.(type) {
	case *ast.Program: // 这里是抽象语法树的根节点
		return evalStatements(node.Statements)

	case *ast.ExpressionStatement: // 解析表达式语句
		return Eval(node.Expression)

	case *ast.IntegerLiteral: // 解析整数字面量
		return &object.Integer{Value: node.Value}

	case *ast.BooleanLiteral: // 解析布尔值
		return nativeBoolToBooleanObject(node.Value)
	}

	return nil
}

func nativeBoolToBooleanObject(input bool) *object.Boolean {
	if input {
		return TRUE
	}
	return FALSE
}

func evalStatements(stmts []ast.Statement) object.Object {
	var result object.Object

	for _, stmt := range stmts {
		result = Eval(stmt)
	}

	return result
}
