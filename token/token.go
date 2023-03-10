package token

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

	// 比较运算符
	LT  // <
	GT  // >
	EQ  // ==
	NEQ // !=
	LE  // <=
	GE  // >=

	// 逻辑运算符
	AND  // &&
	OR   // ||
	BANG // !

	// 位运算符
	BIT_AND      // &
	BIT_OR       // |
	BIT_L_OFFSET // <<
	BIT_R_OFFSET // >>

	// 组合运算符
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

// ErrToken 用于保存错误 token，极其相关信息
type ErrToken struct {
	Type    TokenType
	Literal string
	Msg     string
}

var TokenMap = map[string]TokenType{
	",": COMMA,
	".": DOT,
	":": COLON,
	";": SEMICOLON,
	"_": UNDERLINE,

	"(": LPAREN,
	")": RPAREN,
	"[": LBRACKET,
	"]": RBRACKET,
	"{": LBRACE,
	"}": RBRACE,

	"=": ASSIGN,
	"+": ADD,
	"-": MINUS,
	"*": ASTERISK,
	"/": SLASH,

	"<":  LT,
	">":  GT,
	"==": EQ,
	"!=": NEQ,
	"<=": LE,
	">=": GE,

	"&&": AND,
	"||": OR,
	"!":  BANG,

	"&":  BIT_AND,
	"|":  BIT_OR,
	"<<": BIT_L_OFFSET,
	">>": BIT_R_OFFSET,

	"->": ARROW,

	"func":   FUNC,
	"var":    VAR,
	"bool":   BOOL,
	"true":   TRUE,
	"false":  FALSE,
	"if":     IF,
	"else":   ELSE,
	"meth":   METH,
	"return": RETURN,
}
