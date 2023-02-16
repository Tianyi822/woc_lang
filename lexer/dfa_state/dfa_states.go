package dfa_state

type DfaState uint8

const (
	Initial = DfaState(iota)
	End

	// 基本运算符状态码
	Assign_State // =
	Minus_State  // -

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

	// 数字
	Num_State

	// 标识符
	Ident_State

	// 关键字
	Func_State_1
	Func_State_2
	Func_State_3
	Func_State

	Meth_State_1
	Meth_State_2
	Meth_State_3
	Meth_State

	False_State_2
	False_State_3
	False_State_4
	False_State

	Var_State_1
	Var_State_2
	Var_State

	Bool_State_1
	Bool_State_2
	Bool_State_3
	Bool_State

	True_State_1
	True_State_2
	True_State_3
	True_State

	If_State_1
	If_State

	Else_State_1
	Else_State_2
	Else_State_3
	Else_State

	Return_State_1
	Return_State_2
	Return_State_3
	Return_State_4
	Return_State_5
	Return_State
)
