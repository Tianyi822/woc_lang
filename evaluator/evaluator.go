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

	case *ast.PrefixExpression:
		right := Eval(node.Right) // 解析前缀表达式右侧表达式
		return evalPrefixExpression(node.Operator, right)

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

// evalPrefixExpression 对前缀表达式进行求值
func evalPrefixExpression(operator string, right object.Object) object.Object {
	switch operator {
	case "!":
		return evalBangOperatorExpression(right)
	case "-":
		return evalMinusOperatorExpression(right)
	default:
		return NULL
	}
}

// evalBangOperatorExpression 对取反前缀表达式中的进行求值
func evalBangOperatorExpression(right object.Object) object.Object {
	switch right {
	case TRUE:
		return FALSE
	case FALSE:
		return TRUE
	case NULL:
		return TRUE
	default:
		return FALSE
	}
}

// evalMinusOperatorExpression 对取负前缀表达式中的进行求值
func evalMinusOperatorExpression(right object.Object) object.Object {
	if right.Type() != object.INTEGER_OBJ {
		return NULL
	}

	val := right.(*object.Integer).Value
	return &object.Integer{Value: -val}
}
