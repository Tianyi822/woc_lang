package object

type ObjectType string

const (
	NULL_OBJ    = "NULL"
	INTEGER_OBJ = "INTEGER"
	BOOLEAN_OBJ = "BOOLEAN"
)

// Object 作为 Woclang 中所有对象的始祖，类似与 Java 中的 Object
// 但现在只是初级阶段，暂时不做其他设计
type Object interface {
	Type() ObjectType
	String() string
}
