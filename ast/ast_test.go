package ast

import (
	"testing"
	"woc_lang/token"
)

func TestVarStatement(t *testing.T) {
	stat := &VarStatement{
		Token: token.Token{
			Type:    token.VAR,
			Literal: "var",
		},
		Name: IdentExpression{
			Token: token.Token{
				Type:    token.IDENT,
				Literal: "age",
			},
			Value: "age",
		},
		Value: &IdentExpression{
			Token: token.Token{
				Type:    token.IDENT,
				Literal: "age_value",
			},
			Value: "age_value",
		},
	}

	if stat.String() != "var age = age_value;" {
		t.Errorf("Var 声明语法树错误: %s", stat.String())
	}
}
