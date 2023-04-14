package evaluator

import (
	"woc_lang/ast"
	"woc_lang/object"
)

func Eval(node ast.Node) object.Object {
	switch node := node.(type) {
	case *ast.Program: // 这里是抽象语法树的根节点
		return evalStatements(node.Statements)

	case *ast.ExpressionStatement: // 解析表达式语句
		return Eval(node.Expression)

	case *ast.IntegerLiteral: // 解析整数字面量
		return &object.Integer{Value: node.Value}
	}

	return nil
}

func evalStatements(stmts []ast.Statement) object.Object {
	var result object.Object

	for _, stmt := range stmts {
		result = Eval(stmt)
	}

	return result
}
