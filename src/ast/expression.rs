use crate::ast::{Expression, Node};
use crate::token::Token;

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
        self.token.literal().to_string()
    }
}

impl Expression for BooleanExp {
    fn expression_node(&self) {}
}
