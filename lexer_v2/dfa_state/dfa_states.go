package dfa_state

type DfaState uint8

const (
	Initial = DfaState(iota)
	End

	// 分隔符状态码
	Comma_State
	Dot_State
	Colon_State
	Semicolon_State
	Underline_State

	// 边界符状态码
	Lparen_State   // (
	Rparen_State   // )
	Lbracket_State // [
	Rbracket_State // ]
	Lbrace_State   // {
	Rbrace_State   // }

	// 基本运算符状态码
	Assign_State   // =
	Add_State      // +
	Minus_State    // -
	Asterisk_State // *
	Slash_State    // /

	// 比较运算符
	LT_State   // <
	GT_State   // >
	EQ_State_1 // ==
	EQ_State_2
	LE_State_1 // <=
	LE_State_2
	GE_State_1 // >=
	GE_State_2

	// 逻辑运算符
	AND_State_1 // &&
	AND_State_2
	OR_State_1 // ||
	OR_State_2
	BANG_State // !

	// 位运算符
	BIT_AND_State        // &
	BIT_OR_State         // |
	BIT_L_OFFSET_State_1 // <<
	BIT_L_OFFSET_State_2
	BIT_R_OFFSET_State_1 // >>
	BIT_R_OFFSET_State_2

	Eq_State_1
	Eq_State_2

	If_State_1
	If_State_2

	Else_State_1
	Else_State_2
	Else_State_3
	Else_State_4
)
