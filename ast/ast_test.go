package ast

import (
	"testing"
	"woc_lang/token_v2"
)

func TestVarStatement(t *testing.T) {
	stat := &VarStatement{
		Token: token_v2.Token{
			Type:    token_v2.VAR,
			Literal: "var",
		},
		Name: IdentExpression{
			Token: token_v2.Token{
				Type:    token_v2.IDENT,
				Literal: "age",
			},
			Value: "age",
		},
		Value: &IdentExpression{
			Token: token_v2.Token{
				Type:    token_v2.IDENT,
				Literal: "age_value",
			},
			Value: "age_value",
		},
	}

	if stat.String() != "var age = age_value;" {
		t.Errorf("Var 声明语法树错误: %s", stat.String())
	}
}
