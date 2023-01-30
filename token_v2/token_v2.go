package token_v2

type TokenType uint8

const (
	// ======================== 符号 Token (定义参照 ASCII) ========================
	// 结束符
	EOF = TokenType(iota)

	// 分隔符
	COMMA     // ,
	DOT       // .
	COLON     // :
	SEMICOLON // ;
	UNDERLINE // _

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
	BANG // !
	AND  // &
	OR   // |

	// 组合运算符
	EQ    // ==
	LE    // <=
	GE    // >=
	ARROW // ->

	// 非法符
	ILLEGAL

	// 标识符
	IDENT

	// ======================== 关键字 Token ========================
	// 基本类型关键字
	BOOL
	NUM

	// 语法关键字
	TRUE
	FALSE
	VAR
	IF
	ELSE
	FUNC
	METH
	RETURN
)

// Token 词法分析器解析出来的词法对象
type Token struct {
	// Type 是 Token 的类型，例如 x = 5
	// x 是标识符类型，而 5 是一个数值类型
	Type TokenType
	// Literal 是 Token 的字面值，例如 x = 5
	// token x 的字面量就是 x，类型是 IDENT
	// token 5 的字面量就是 5，类型是 NUM
	Literal string
}
