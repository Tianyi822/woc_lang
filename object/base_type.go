package object

import "fmt"

// 空值
type NULL struct{}

func (n *NULL) Type() ObjectType {
	return NULL_OBJ
}

func (n *NULL) String() string {
	return "null"
}

// Integer 整型
type Integer struct {
	Value int64
}

func (i *Integer) Type() ObjectType {
	return INTEGER_OBJ
}

func (i *Integer) String() string {
	return fmt.Sprintf("%d", i.Value)
}

// Boolean 布尔值
type Boolean struct {
	Value bool
}

func (b *Boolean) Type() ObjectType {
	return BOOLEAN_OBJ
}

func (b *Boolean) String() string {
	return fmt.Sprintf("%t", b.Value)
}
