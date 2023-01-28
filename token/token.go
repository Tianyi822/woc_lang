package token

type TokenType uint8 // Token 类型

const (
	// ======================== 符号 Token (定义参照 ASCII) ========================
	// 非打印控制符
	TAB   TokenType = 9  // 制表符
	LF    TokenType = 10 // 换行符
	CR    TokenType = 13 // 回车符
	BLANK TokenType = 32 // 空白符

	// 分隔符
	COMMA     TokenType = 44 // ,
	DOT       TokenType = 46 // .
	COLON     TokenType = 58 // :
	SEMICOLON TokenType = 59 // ;
	UNDERLINE TokenType = 95 // _

	// 边界符
	LPAREN   TokenType = 40  // (
	RPAREN   TokenType = 41  // )
	LBRACKET TokenType = 91  // [
	RBRACKET TokenType = 93  // ]
	LBRACE   TokenType = 123 // {
	RBRACE   TokenType = 125 // }

	// 基本运算符
	ASSIGN   TokenType = 61 // =
	ADD      TokenType = 43 // +
	MINUS    TokenType = 45 // -
	ASTERISK TokenType = 42 // *
	SLASH    TokenType = 47 // /

	// 布尔运算符
	LT   TokenType = 60  // <
	GT   TokenType = 62  // >
	BANG TokenType = 33  // !
	AND  TokenType = 38  // &
	OR   TokenType = 124 // |

	// 结束符
	END_MARK TokenType = 0

	// 组合运算符
	EQ = TokenType(iota + 127) // ==
	LE                         // <=
	GE                         // >=

	// 非法符
	ILLEGAL

	// 标识符
	IDENT

	// ======================== 关键字 Token ========================
	// 基本类型关键字
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
