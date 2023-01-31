package dfa_state

type DfaState uint8

const (
	Initial = DfaState(iota)

	Eq_State_1
	Eq_State_2

	If_State_1
	If_State_2

	Else_State_1
	Else_State_2
	Else_State_3
	Else_State_4
)
