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

	case *ast.BlockStatement: // 解析代码块
		return evalStatements(node.Statements)

	case *ast.IfExpression: // 解析 if 表达式
		return evalIfExpression(node)

	case *ast.PrefixExpression:
		right := Eval(node.Right) // 解析前缀表达式右侧表达式
		return evalPrefixExpression(node.Operator, right)

	case *ast.InfixExpression:
		left := Eval(node.LeftExp)   // 解析中缀表达式左侧表达式
		right := Eval(node.RightExp) // 解析中缀表达式右侧表达式
		return evalInfixExpression(node.Operator, left, right)

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

// evalIfExpression 对 if 表达式进行求值
func evalIfExpression(ie *ast.IfExpression) object.Object {
	condition := Eval(ie.Condition) // 解析 if 表达式条件

	if isTruthy(condition) { // 如果条件为真
		return Eval(ie.Consequence) // 解析 if 表达式的主体
	} else if ie.ElseExpression != nil { // 如果条件为假且有 else 表达式
		return evalElseExpression(ie.ElseExpression) // 解析 else 表达式
	}

	return NULL
}

// evalElseExpression 对 else 表达式进行求值
func evalElseExpression(ee *ast.ElseExpression) object.Object {
	if ee.NextIfExp == nil { // 如果没有下一个 if 表达式
		return Eval(ee.Consequence) // 解析 else 表达式的主体
	} else { // 如果有下一个 if 表达式
		return evalIfExpression(ee.NextIfExp) // 解析下一个 if 表达式
	}
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

func evalInfixExpression(operator string, left, right object.Object) object.Object {
	switch {
	case left.Type() == object.INTEGER_OBJ && right.Type() == object.INTEGER_OBJ:
		return evalIntegerInfixExpression(operator, left, right)
	case operator == "==":
		return nativeBoolToBooleanObject(left == right)
	case operator == "!=":
		return nativeBoolToBooleanObject(left != right)
	default:
		return NULL
	}
}

func evalIntegerInfixExpression(operator string, left, right object.Object) object.Object {
	leftVal := left.(*object.Integer).Value
	rightVal := right.(*object.Integer).Value

	switch operator {
	case "+":
		return &object.Integer{Value: leftVal + rightVal}
	case "-":
		return &object.Integer{Value: leftVal - rightVal}
	case "*":
		return &object.Integer{Value: leftVal * rightVal}
	case "/":
		return &object.Integer{Value: leftVal / rightVal}
	case "<":
		return nativeBoolToBooleanObject(leftVal < rightVal)
	case ">":
		return nativeBoolToBooleanObject(leftVal > rightVal)
	case "==":
		return nativeBoolToBooleanObject(leftVal == rightVal)
	case "!=":
		return nativeBoolToBooleanObject(leftVal != rightVal)
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

// isTruthy 判断一个对象是否为真
func isTruthy(obj object.Object) bool {
	switch obj {
	case NULL:
		return false
	case TRUE:
		return true
	case FALSE:
		return false
	default:
		return true
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
