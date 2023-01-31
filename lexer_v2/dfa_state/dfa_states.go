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
	Lt_State  // <
	Gt_State  // >
	Eq_State  // ==
	Neq_State // !=
	Le_State  // <=
	Ge_State  // >=

	// 逻辑运算符
	And_State  // &&
	Or_State   // ||
	Bang_State // !

	// 位运算符
	Bit_And_State      // &
	Bit_Or_State       // |
	Bit_L_Offset_State // <<
	Bit_R_Offset_State // >>

	// 组合运算符
	Arrow_State

	If_State_1
	If_State_2

	Else_State_1
	Else_State_2
	Else_State_3
	Else_State_4
)
