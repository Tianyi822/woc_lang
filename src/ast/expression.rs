use crate::ast::ast::{Expression, Node};
use crate::token::token::Token;

use super::statement::BlockStatement;

// This struct is used to represent the prefix expression: !5, -15. etc.
pub struct PrefixExp {
    token: Token,
    operator: String,
    right: Box<dyn Expression>,
}

// This struct is used to represent the prefix expression: !5, -55. etc.
impl PrefixExp {
    pub fn new(token: Token, operator: String, right: Box<dyn Expression>) -> PrefixExp {
        PrefixExp {
            token,
            operator,
            right,
        }
    }
}

impl Node for PrefixExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        format!("({}{})", self.operator, self.right.to_string())
    }
}

impl Expression for PrefixExp {
    fn expression_node(&self) {}
}

pub struct InfixExp {
    token: Token,
    left: Box<dyn Expression>,
    operator: String,
    right: Box<dyn Expression>,
}

// This struct is used to represent the infix expression: 5 + 5, 5 - 5, etc.
impl InfixExp {
    pub fn new(
        token: Token,
        left: Box<dyn Expression>,
        operator: String,
        right: Box<dyn Expression>,
    ) -> Self {
        Self {
            token,
            left,
            operator,
            right,
        }
    }
}

impl Node for InfixExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}

impl Expression for InfixExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the identifier expression.
pub struct IdentifierExp {
    token: Token,
    value: String,
}

impl IdentifierExp {
    pub fn new(token: Token, value: String) -> IdentifierExp {
        IdentifierExp { token, value }
    }
}

impl Node for IdentifierExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for IdentifierExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the number expression.
pub struct NumExp {
    token: Token,
    i_value: Option<i64>,
    f_value: Option<f64>,
}

impl NumExp {
    pub fn new(token: Token, i_value: Option<i64>, f_value: Option<f64>) -> NumExp {
        NumExp {
            token,
            i_value,
            f_value,
        }
    }
}

impl Node for NumExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        match self.i_value {
            Some(i) => i.to_string(),
            None => self.f_value.unwrap().to_string(),
        }
    }
}

impl Expression for NumExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the boolean expression.
pub struct BooleanExp {
    token: Token,
    value: bool,
}

impl BooleanExp {
    pub fn new(token: Token, value: bool) -> BooleanExp {
        BooleanExp { token, value }
    }
}

impl Node for BooleanExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for BooleanExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the if expression.
pub struct IfExp {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    else_exp: Option<Box<ElseExp>>,
}

impl IfExp {
    pub fn new(
        token: Token,
        condition: Box<dyn Expression>,
        consequence: BlockStatement,
        else_exp: Option<Box<ElseExp>>,
    ) -> IfExp {
        IfExp {
            token,
            condition,
            consequence,
            else_exp,
        }
    }
}

impl Node for IfExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str("if ");
        out.push_str(&self.condition.to_string());
        out.push_str(" ");
        out.push_str(&self.consequence.to_string());

        if self.else_exp.is_some() {
            out.push_str(&self.else_exp.as_ref().unwrap().to_string());
        }

        out
    }
}

impl Expression for IfExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the else expression.
pub struct ElseExp {
    token: Token,
    if_exp: Option<Box<dyn Expression>>,
    alternative: Option<BlockStatement>,
}

impl ElseExp {
    pub fn new(
        token: Token,
        if_exp: Option<Box<dyn Expression>>,
        alternative: Option<BlockStatement>,
    ) -> ElseExp {
        ElseExp {
            token,
            if_exp,
            alternative,
        }
    }
}

impl Node for ElseExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(" else ");

        if self.if_exp.is_some() {
            out.push_str(&self.if_exp.as_ref().unwrap().to_string());
        }

        if self.alternative.is_some() {
            out.push_str(&self.alternative.as_ref().unwrap().to_string());
        }

        out
    }
}

impl Expression for ElseExp {
    fn expression_node(&self) {}
}

// This struct is used to represent the function expression.
pub struct FunctionExp {
    token: Token,
    name: IdentifierExp,
    parameters: Option<Vec<IdentifierExp>>,
    body: BlockStatement,
}

impl FunctionExp {
    pub fn new(
        token: Token,
        name: IdentifierExp,
        parameters: Option<Vec<IdentifierExp>>,
        body: BlockStatement,
    ) -> FunctionExp {
        FunctionExp {
            token,
            name,
            parameters,
            body,
        }
    }
}

impl Node for FunctionExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        // func <name>(<parameters>) <body>
        // func token
        out.push_str(&self.token_literal());
        out.push_str(" ");
        // <name>
        out.push_str(&self.name.to_string());
        out.push_str(" ");

        // <parameters>
        out.push_str("(");
        let mut params_str = String::new();
        if self.parameters.is_some() {
            // collect parameters and separate them with a comma.
            for (i, param) in self.parameters.as_ref().unwrap().iter().enumerate() {
                if i > 0 {
                    params_str.push_str(", ");
                }
                params_str.push_str(&param.to_string());
            }
        }
        out.push_str(&params_str);
        out.push_str(") ");

        // <body>
        out.push_str(&self.body.to_string());

        out
    }
}

impl Expression for FunctionExp {
    fn expression_node(&self) {}
}

pub struct CallExp {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Option<Vec<Box<dyn Expression>>>,
}

impl CallExp {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Option<Vec<Box<dyn Expression>>>,
    ) -> CallExp {
        CallExp {
            token,
            function,
            arguments,
        }
    }
}

impl Node for CallExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.function.to_string());
        out.push_str("(");

        let mut args_str = String::new();
        if self.arguments.is_some() {
            let args = self.arguments.as_ref().unwrap();
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    args_str.push_str(", ");
                }
                args_str.push_str(&arg.to_string());
            }
        }

        out.push_str(&args_str);
        out.push_str(")");

        out
    }
}

impl Expression for CallExp {
    fn expression_node(&self) {}
}
