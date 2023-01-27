package token

type TokenType byte // Token 类型

const (
	// 分隔符
	COMMA     = TokenType(iota) // ,
	DOT                         // .
	COLON                       // :
	SEMICOLON                   // ;

	// 边界符
	LPAREN   // (
	RPAREN   // )
	LBRACKET // [
	RBRACKET // ]
	LBRACE   // {
	RBRACE   // }

	// 基本运算符
	ASSIGN   // =
	ADD      // +
	MINUS    // -
	ASTERISK // *
	SLASH    // /

	// 布尔运算符
	LT   // <
	GT   // >
	LE   // <=
	GE   // >=
	EQ   // ==
	NEQ  // !=
	BANG // !
	AND  // &
	OR   // |

	// TODO: 位运算符

	// 结束符
	END_MARK

	// 非法符
	ILLEGAL

	// 标识符
	IDENT

	// 基本类型
	BYTE
	I32
	I64
	// TODO: 后续添加浮点以及无符号类型

	// 语法关键字
	TRUE
	FALSE
	VAR
	IF
	ELSE
	FUNC
	METH
)

// Token 词法分析器识别出来的词法对象
type Token struct {
	// Type 是 Token 的类型，例如 x = 5
	// x 是标识符类型，而 5 是一个数值类型
	Type TokenType
	// Literal 是 Token 的字面值，例如 x = 5
	// token x 的字面量就是 x，类型是 _IDENT
	// token 5 的字面量就是 5，类型是 _I32
	Literal string
}

var kwTable = map[string]TokenType{
	"true":  TRUE,
	"false": FALSE,
	"var":   VAR,
	"if":    IF,
	"else":  ELSE,
	"func":  FUNC,
	"meth":  METH,
}
