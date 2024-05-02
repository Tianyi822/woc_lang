use crate::ast_v2::{Expression, Node, Statement};
use crate::ast_v2::expressions::{CallExp, ElseExp, IdentifierExp, IfExp, InfixExp, PrefixExp};
use crate::ast_v2::statements::{BlockStatement, FuncStatement, LetStatement, ReturnStatement};
use crate::evaluator_v2::scope::scope::Scope;
use crate::object::object::{BaseValue, Function, Object, Value};
use crate::token::token::TokenType;

#[derive(Clone)]
pub struct Evaluator {
    scope: Scope,
}

impl Evaluator {
    pub fn new(parent_scope: Option<Box<Scope>>) -> Self {
        Self {
            scope: Scope::new(parent_scope),
        }
    }

    pub fn scope(&self) -> &Scope {
        &self.scope
    }

    pub fn eval(&self, node: &Node) -> Object {
        match node {
            Node::Exp(exp) => self.eval_exp(exp),
            Node::Stmt(stmt) => self.eval_stmt(stmt),
        }
    }

    fn eval_exp(&self, exp: &Expression) -> Object {
        match exp {
            Expression::Num(num) => match num.integer_value() {
                Some(value) => Object::Base(BaseValue::Integer(Value::new(value))),
                None => Object::Base(BaseValue::Float(Value::new(num.float_value().unwrap()))),
            },
            Expression::Boolean(b) => Object::Base(BaseValue::Boolean(Value::new(b.value()))),
            Expression::Identifier(ident_exo) => self.eval_ident_exp(ident_exo),
            Expression::Prefix(pre_exp) => self.eval_prefix_exp(pre_exp),
            Expression::Infix(infix_exp) => self.eval_infix_exp(infix_exp),
            Expression::If(if_exp) => self.eval_if_exp(if_exp),
            Expression::Call(call_exp) => self.eval_call_exp(call_exp),
        }
    }

    fn eval_stmt(&self, stmt: &Statement) -> Object {
        match stmt {
            Statement::Let(let_stmt) => self.eval_let_stmt(let_stmt),
            Statement::Return(ret_stmt) => self.eval_return_stmt(ret_stmt),
            Statement::Block(block_stmt) => self.eval_block_stmt(block_stmt),
            Statement::Func(func_stmt) => self.eval_func_stmt(func_stmt),
        }
    }

    // =================== Evaluate Statement ===================

    fn eval_let_stmt(&self, stmt: &LetStatement) -> Object {
        let value = match self.eval_exp(stmt.value().unwrap()) {
            Object::Return(v) => *v,
            v => v,
        };
        self.scope.set(stmt.name().to_string(), value);
        Object::Null
    }

    fn eval_return_stmt(&self, stmt: &ReturnStatement) -> Object {
        let ret_val = match stmt.value() {
            Some(v) => self.eval_exp(v),
            None => Object::Null,
        };

        match ret_val {
            Object::Base(BaseValue::Integer(v)) => {
                Object::Return(Box::new(Object::Base(BaseValue::Integer(v))))
            }
            Object::Base(BaseValue::Float(v)) => {
                Object::Return(Box::new(Object::Base(BaseValue::Float(v))))
            }
            Object::Base(BaseValue::Boolean(v)) => {
                Object::Return(Box::new(Object::Base(BaseValue::Boolean(v))))
            }
            _ => Object::Null,
        }
    }

    fn eval_func_stmt(&self, func_stmt: &FuncStatement) -> Object {
        let params = match func_stmt.params() {
            Some(p) => {
                let params: Vec<IdentifierExp> = p.clone();
                Some(params)
            }
            None => None,
        };

        let body = func_stmt.body().clone();
        let func = Object::Func(Function::new(
            params,
            body,
            Some(Box::new(self.scope.clone())),
        ));

        self.scope.set(func_stmt.name().to_string(), func);

        Object::Null
    }

    pub fn eval_block_stmt(&self, stmt: &BlockStatement) -> Object {
        let mut result = Object::Null;
        match stmt.statements() {
            Some(stmts) => {
                for s in stmts {
                    result = self.eval(s);

                    match result {
                        Object::Return(_) => return result,
                        _ => {}
                    }
                }
            }
            None => {
                result = Object::Null;
            }
        }
        result
    }

    // =================== Evaluate Expression ===================

    fn eval_ident_exp(&self, exp: &IdentifierExp) -> Object {
        let name = exp.value();
        match self.scope.get(name) {
            Some(v) => {
                return v.as_ref().clone();
            }
            _ => Object::Null,
        }
    }

    fn eval_if_exp(&self, if_exp: &IfExp) -> Object {
        let condition = self.eval_exp(if_exp.condition());

        if self.is_truthy(&condition) {
            // if condition is true
            return self.eval_block_stmt(if_exp.consequence());
        } else if if_exp.else_exp().is_some() {
            // if condition is false and there is an else expression
            return self.eval_else_exp(if_exp.else_exp().unwrap());
        } else {
            // if condition is false and there is no else expression
            return Object::Null;
        }
    }

    fn eval_else_exp(&self, else_exp: &ElseExp) -> Object {
        if else_exp.if_exp().is_some() {
            return self.eval_if_exp(else_exp.if_exp().unwrap());
        } else {
            return self.eval_block_stmt(else_exp.consequence().unwrap());
        }
    }

    fn eval_call_exp(&self, call_exp: &CallExp) -> Object {
        match self.scope.get(call_exp.name().value()) {
            Some(v) => {
                let func = v.as_ref().clone();
                match func {
                    Object::Func(mut f) => {
                        // Get the arguments of the function
                        let arguments: Vec<Object> = call_exp
                            .arguments()
                            .iter()
                            .map(|a| self.eval_exp(a))
                            .collect();
                        match f.add_arguments(arguments) {
                            Ok(_) => {
                                // Evaluate the body of the function
                                return f.eval();
                            }
                            Err(_) => return Object::Null,
                        }
                    }
                    _ => return Object::Null,
                }
            }
            _ => return Object::Null,
        }
    }

    fn eval_prefix_exp(&self, pre_exp: &PrefixExp) -> Object {
        match pre_exp.operator() {
            TokenType::Not => {
                let right = self.eval_exp(pre_exp.right());
                match right {
                    Object::Base(BaseValue::Boolean(v)) => {
                        Object::Base(BaseValue::Boolean(Value::new(!v.value())))
                    }
                    Object::Base(BaseValue::Integer(v)) => {
                        Object::Base(BaseValue::Boolean(Value::new(v.is_zero())))
                    }
                    Object::Base(BaseValue::Float(v)) => {
                        Object::Base(BaseValue::Boolean(Value::new(v.is_zero())))
                    }
                    _ => Object::Null,
                }
            }
            TokenType::Minus => {
                let right = self.eval_exp(pre_exp.right());
                match right {
                    Object::Base(BaseValue::Integer(v)) => {
                        Object::Base(BaseValue::Integer(Value::new(-v.value())))
                    }
                    Object::Base(BaseValue::Float(v)) => {
                        Object::Base(BaseValue::Float(Value::new(-v.value())))
                    }
                    _ => Object::Null,
                }
            }
            _ => Object::Null,
        }
    }

    fn eval_infix_exp(&self, infix_exp: &InfixExp) -> Object {
        let left = match self.eval_exp(infix_exp.left()) {
            Object::Return(v) => *v,
            v => v,
        };
        let right = match self.eval_exp(infix_exp.right()) {
            Object::Return(v) => *v,
            v => v,
        };

        match infix_exp.operator() {
            TokenType::And
            | TokenType::Or
            | TokenType::EqualTo
            | TokenType::NotEqualTo
            | TokenType::Less
            | TokenType::LessThanOrEqualTo
            | TokenType::Greater
            | TokenType::GreaterThanOrEqualTo => match (left, right) {
                // Boolean compare with Boolean
                (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Boolean(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() && *r.value())))
                        }
                        TokenType::Or => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() || *r.value())))
                        }
                        TokenType::EqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                        }
                        TokenType::NotEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Boolean compare with Integer
                (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Integer(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() && !r.is_zero())))
                        }
                        TokenType::Or => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() || !r.is_zero())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Integer compare with Boolean
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Boolean(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => {
                            Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && *r.value())))
                        }
                        TokenType::Or => {
                            Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || *r.value())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Boolean compare with Float
                (Object::Base(BaseValue::Boolean(l)), Object::Base(BaseValue::Float(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() && !r.is_zero())))
                        }
                        TokenType::Or => {
                            Object::Base(BaseValue::Boolean(Value::new(*l.value() || !r.is_zero())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Float compare with Boolean
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Boolean(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => {
                            Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() && *r.value())))
                        }
                        TokenType::Or => {
                            Object::Base(BaseValue::Boolean(Value::new(!l.is_zero() || *r.value())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Integer compare with Integer
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Integer(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => Object::Base(BaseValue::Boolean(Value::new(
                            !l.is_zero() && !r.is_zero(),
                        ))),
                        TokenType::Or => Object::Base(BaseValue::Boolean(Value::new(
                            !l.is_zero() || !r.is_zero(),
                        ))),
                        TokenType::EqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                        }
                        TokenType::NotEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                        }
                        TokenType::Less => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() < r.value())))
                        }
                        TokenType::LessThanOrEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() <= r.value())))
                        }
                        TokenType::Greater => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() > r.value())))
                        }
                        TokenType::GreaterThanOrEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() >= r.value())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Float compare with Float
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Float(r))) => {
                    match infix_exp.operator() {
                        TokenType::And => Object::Base(BaseValue::Boolean(Value::new(
                            !l.is_zero() && !r.is_zero(),
                        ))),
                        TokenType::Or => Object::Base(BaseValue::Boolean(Value::new(
                            !l.is_zero() || !r.is_zero(),
                        ))),
                        TokenType::EqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() == r.value())))
                        }
                        TokenType::NotEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() != r.value())))
                        }
                        TokenType::Less => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() < r.value())))
                        }
                        TokenType::LessThanOrEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() <= r.value())))
                        }
                        TokenType::Greater => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() > r.value())))
                        }
                        TokenType::GreaterThanOrEqualTo => {
                            Object::Base(BaseValue::Boolean(Value::new(l.value() >= r.value())))
                        }
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Float compare with Integer
                (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Integer(r))) => {
                    match infix_exp.operator() {
                        TokenType::EqualTo => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() == *r.value() as f64,
                        ))),
                        TokenType::NotEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() != *r.value() as f64,
                        ))),
                        TokenType::Less => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() < *r.value() as f64,
                        ))),
                        TokenType::LessThanOrEqualTo => Object::Base(BaseValue::Boolean(
                            Value::new(*l.value() <= *r.value() as f64),
                        )),
                        TokenType::Greater => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() > *r.value() as f64,
                        ))),
                        TokenType::GreaterThanOrEqualTo => Object::Base(BaseValue::Boolean(
                            Value::new(*l.value() >= *r.value() as f64),
                        )),
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }

                // Integer compare with Float
                (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Float(r))) => {
                    match infix_exp.operator() {
                        TokenType::EqualTo => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() as f64 == *r.value(),
                        ))),
                        TokenType::NotEqualTo => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() as f64 != *r.value(),
                        ))),
                        TokenType::Less => Object::Base(BaseValue::Boolean(Value::new(
                            (*l.value() as f64) < *r.value(),
                        ))),
                        TokenType::LessThanOrEqualTo => Object::Base(BaseValue::Boolean(
                            Value::new(*l.value() as f64 <= *r.value()),
                        )),
                        TokenType::Greater => Object::Base(BaseValue::Boolean(Value::new(
                            *l.value() as f64 > *r.value(),
                        ))),
                        TokenType::GreaterThanOrEqualTo => Object::Base(BaseValue::Boolean(
                            Value::new(*l.value() as f64 >= *r.value()),
                        )),
                        _ => Object::Base(BaseValue::Boolean(Value::new(false))),
                    }
                }
                _ => Object::Base(BaseValue::Boolean(Value::new(false))),
            },

            TokenType::Plus | TokenType::Minus | TokenType::Asterisk | TokenType::Slash => {
                match (left, right) {
                    (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Integer(r))) => {
                        match infix_exp.operator() {
                            TokenType::Plus => {
                                Object::Base(BaseValue::Integer(Value::new(l.value() + r.value())))
                            }
                            TokenType::Minus => {
                                Object::Base(BaseValue::Integer(Value::new(l.value() - r.value())))
                            }
                            TokenType::Asterisk => {
                                Object::Base(BaseValue::Integer(Value::new(l.value() * r.value())))
                            }
                            TokenType::Slash => {
                                Object::Base(BaseValue::Integer(Value::new(l.value() / r.value())))
                            }
                            _ => Object::Null,
                        }
                    }
                    (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Float(r))) => {
                        match infix_exp.operator() {
                            TokenType::Plus => {
                                Object::Base(BaseValue::Float(Value::new(l.value() + r.value())))
                            }
                            TokenType::Minus => {
                                Object::Base(BaseValue::Float(Value::new(l.value() - r.value())))
                            }
                            TokenType::Asterisk => {
                                Object::Base(BaseValue::Float(Value::new(l.value() * r.value())))
                            }
                            TokenType::Slash => {
                                Object::Base(BaseValue::Float(Value::new(l.value() / r.value())))
                            }
                            _ => Object::Null,
                        }
                    }
                    (Object::Base(BaseValue::Integer(l)), Object::Base(BaseValue::Float(r))) => {
                        match infix_exp.operator() {
                            TokenType::Plus => Object::Base(BaseValue::Float(Value::new(
                                *l.value() as f64 + r.value(),
                            ))),
                            TokenType::Minus => Object::Base(BaseValue::Float(Value::new(
                                *l.value() as f64 - r.value(),
                            ))),
                            TokenType::Asterisk => Object::Base(BaseValue::Float(Value::new(
                                *l.value() as f64 * r.value(),
                            ))),
                            TokenType::Slash => Object::Base(BaseValue::Float(Value::new(
                                *l.value() as f64 / r.value(),
                            ))),
                            _ => Object::Null,
                        }
                    }
                    (Object::Base(BaseValue::Float(l)), Object::Base(BaseValue::Integer(r))) => {
                        match infix_exp.operator() {
                            TokenType::Plus => Object::Base(BaseValue::Float(Value::new(
                                l.value() + *r.value() as f64,
                            ))),
                            TokenType::Minus => Object::Base(BaseValue::Float(Value::new(
                                l.value() - *r.value() as f64,
                            ))),
                            TokenType::Asterisk => Object::Base(BaseValue::Float(Value::new(
                                l.value() * *r.value() as f64,
                            ))),
                            TokenType::Slash => Object::Base(BaseValue::Float(Value::new(
                                l.value() / *r.value() as f64,
                            ))),
                            _ => Object::Null,
                        }
                    }
                    _ => Object::Null,
                }
            }
            _ => Object::Null,
        }
    }

    // =================== Helper Functions ===================

    fn is_truthy(&self, obj: &Object) -> bool {
        match obj {
            Object::Base(BaseValue::Boolean(v)) => *v.value(),
            Object::Base(BaseValue::Integer(v)) => !v.is_zero(),
            Object::Base(BaseValue::Float(v)) => !v.is_zero(),
            _ => false,
        }
    }
}
