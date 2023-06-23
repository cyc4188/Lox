use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use super::*;

pub struct Interpreter {
    environment: EnvironmentRef,
    pub globals: EnvironmentRef,
    pub locals: HashMap<Token, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        let clock: Object = Object::Callable(Function::Native {
            name: "clock".to_string(),
            arity: 0,
            body: Box::new(|_: &Vec<Object>| -> Object {
                Object::Number(NumberType::Float(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                ))
            }),
        });
        globals.borrow_mut().define("clock", clock);

        Self {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, Error> {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        environment: EnvironmentRef,
    ) -> Result<(), Error> {
        let previous = self.environment.clone();
        self.environment = environment;
        let mut steps = || -> Result<(), Error> {
            for statement in stmts {
                self.execute(statement)?;
            }
            Ok(())
        };
        let result = steps();
        self.environment = previous;
        result
    }

    fn number_operand_error(&self, operator: &Token) -> Result<Object, Error> {
        Err(Error {
            message: format!("Operand of {} must be a number.", operator.token_type),
            error_type: ErrorType::RuntimeError(operator.clone()),
        })
    }

    fn is_truthy(object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }

    fn is_equal(left: &Object, right: &Object) -> bool {
        left.equals(right)
    }

    fn stringify(object: &Object) -> String {
        match object {
            Object::Nil => "nil".to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::Number(n) => n.to_string(),
            Object::String(s) => s.clone(),
            Object::Callable(function) => function.to_string(),
            Object::Class(class) => class.borrow().to_string(),
            Object::Instance(instance) => instance.borrow().to_string(),
            Object::List(list) => list.borrow().to_string(),
        }
    }
    fn check_integer(obj: &Object) -> Option<i64> {
        if let Object::Number(NumberType::Integer(n)) = obj {
            Some(*n)
        } else {
            None
        }
    }

    pub fn resolve(&mut self, token: &Token, depth: usize) {
        trace!("Resolving {} at depth {}", token.lexeme, depth);
        self.locals.insert(token.clone(), depth);
    }

    fn look_up_variable(&self, name: &Token) -> Result<Object, Error> {
        let result: Option<Object> = match self.locals.get(name) {
            Some(distance) => self.environment.borrow().get_at(*distance, &name.lexeme),
            None => self.globals.borrow().get(&name.lexeme),
        };
        if let Some(obj) = result {
            Ok(obj)
        } else {
            Err(Error {
                message: format!("Undefined variable {}.", name.lexeme),
                error_type: ErrorType::RuntimeError(name.clone()),
            })
        }
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Object, Error> {
        match value {
            Literal::Boolean(b) => Ok(Object::Boolean(*b)),
            Literal::Nil => Ok(Object::Nil),
            Literal::Number(n) => Ok(Object::Number(*n)), // TODO
            Literal::String(s) => Ok(Object::String(s.clone())),
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;

                // -, !
                match operator.token_type {
                    TokenType::Minus => {
                        match right {
                            // check if right is a number
                            Object::Number(n) => Ok(Object::Number(n.unary_op(operator)?)),
                            _ => self.number_operand_error(operator),
                        }
                    }
                    TokenType::Bang => Ok(Object::Boolean(!Interpreter::is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Number(l.binary_op(operator, &r)?))
                        }
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Plus => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Number(l.binary_op(operator, &r)?))
                        }
                        (Object::String(l), Object::String(r)) => Ok(Object::String(l + &r)),
                        _ => Err(Error {
                            message: format!(
                                "Operands of {} must be two numbers or two strings.",
                                operator.token_type
                            ),
                            error_type: ErrorType::RuntimeError(operator.clone()),
                        }),
                    },
                    TokenType::Slash => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Number(l.binary_op(operator, &r)?))
                        }
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Star => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Number(l.binary_op(operator, &r)?))
                        }
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Greater => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Boolean(l.greater(&r)?))
                        }
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l > r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Boolean(l.greater_equal(&r)?))
                        }
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l >= r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::Less => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(l.less(&r)?)),
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l < r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Object::Number(l), Object::Number(r)) => {
                            Ok(Object::Boolean(l.less_equal(&r)?))
                        }
                        (Object::String(l), Object::String(r)) => Ok(Object::Boolean(l <= r)),
                        _ => self.number_operand_error(operator),
                    },
                    TokenType::BangEqual => {
                        Ok(Object::Boolean(!Interpreter::is_equal(&left, &right)))
                    }

                    TokenType::EqualEqual => {
                        Ok(Object::Boolean(Interpreter::is_equal(&left, &right)))
                    }

                    TokenType::And => {
                        if !Interpreter::is_truthy(&left) {
                            Ok(left)
                        } else {
                            Ok(right)
                        }
                    }

                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Grouping { expression } => self.evaluate(expression),
            _ => unreachable!(),
        }
    }
    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Variable { name } => self.look_up_variable(name),
            _ => unreachable!(),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;

                let distance = self.locals.get(name);
                if let Some(distance) = distance {
                    self.environment
                        .borrow_mut()
                        .assign_at(*distance, name, &value)?;
                    Ok(value)
                } else {
                    self.environment.borrow_mut().assign(name, &value)?;
                    Ok(value)
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_logic_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(left)?;
                if operator.token_type == TokenType::Or {
                    if Interpreter::is_truthy(&left_value) {
                        Ok(left_value)
                    } else {
                        Ok(self.evaluate(right)?)
                    }
                } else if Interpreter::is_truthy(&left_value) {
                    Ok(self.evaluate(right)?)
                } else {
                    Ok(left_value)
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_index_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        trace!("visit_index_expr: {}", expr);
        match expr {
            Expr::Index {
                object: left,
                operator,
                index,
                index_end,
            } => {
                // evaluate left
                let left = self.evaluate(left)?;
                let index = self.evaluate(index)?;
                let index_end = match index_end {
                    Some(index_end) => Some(self.evaluate(index_end)?),
                    None => None,
                };
                let start: i64;
                // check if right is a Number
                if let Some(n) = Interpreter::check_integer(&index) {
                    start = n;
                } else {
                    return Err(Error {
                        message: format!("Expected integer got {}", index),
                        error_type: ErrorType::RuntimeError(operator.clone()),
                    });
                }

                let mut end: i64 = start + 1;
                if let Some(index_end) = index_end {
                    if let Some(n) = Interpreter::check_integer(&index_end) {
                        end = n;
                    } else {
                        return Err(Error {
                            message: format!("Expected integer got {}", index_end),
                            error_type: ErrorType::RuntimeError(operator.clone()),
                        });
                    }
                }

                // check if left is a String
                if let Object::String(s) = left {
                    // check if nth is in range
                    if s.len() <= start as usize || start < 0 {
                        return Err(Error {
                            message: format!("Index out of range: {}", start),
                            error_type: ErrorType::RuntimeError(operator.clone()),
                        });
                    }
                    if s.len() < end as usize || end < 0 {
                        return Err(Error {
                            message: format!("Index out of range: {}", end),
                            error_type: ErrorType::RuntimeError(operator.clone()),
                        });
                    }

                    // 空串
                    if start >= end {
                        return Ok(Object::String("".to_string()));
                    }

                    // return the substr
                    return Ok(Object::String(
                        s.chars()
                            .skip(start as usize)
                            .take((end - start) as usize)
                            .collect(),
                    ));
                }
                Err(Error {
                    message: format!("Expected string got {}", left),
                    error_type: ErrorType::RuntimeError(operator.clone()),
                })
                // check if left if an Array TOOD
            }
            _ => unreachable!(),
        }
    }
    fn visit_call_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        trace!("visit_call_expr");
        match expr {
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;

                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.evaluate(arg)?);
                }

                // check if callee is a function
                if let Object::Callable(function) = callee {
                    // check if number of arguments matches number of parameters
                    trace!("function arity: {}", function.arity(),);
                    trace!("args.len: {}", args.len());
                    if function.arity() != args.len() {
                        return Err(Error {
                            message: format!(
                                "Expected {} arguments but got {}.",
                                function.arity(),
                                args.len()
                            ),
                            error_type: ErrorType::RuntimeError(paren.clone()),
                        });
                    }
                    // call function
                    Ok(function.call(self, &args)?)
                } else if let Object::Class(class) = callee {
                    // call class init
                    // get a new instance of the class
                    let instance =
                        Object::Instance(Rc::new(RefCell::new(LoxInstance::new(class.clone()))));
                    if let Some(initializer) = class.borrow().get_method("init") {
                        if initializer.arity() != args.len() {
                            return Err(Error {
                                message: format!(
                                    "Expected {} arguments but got {}.",
                                    initializer.arity(),
                                    args.len()
                                ),
                                error_type: ErrorType::RuntimeError(paren.clone()),
                            });
                        }
                        initializer.bind(instance.clone()).call(self, &args)?;
                    }

                    Ok(instance)
                } else {
                    Err(Error {
                        message: "Can only call functions and classes.".to_string(),
                        error_type: ErrorType::RuntimeError(paren.clone()),
                    })
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_get_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Get { object, name } => {
                let object = object.accept(self)?;
                if let Object::Instance(ref instance) = object {
                    let field = instance.borrow().get(&name.lexeme, &object);
                    if let Some(field) = field {
                        Ok(field)
                    } else {
                        Err(Error {
                            message: format!("Undefined property '{}'.", name.lexeme),
                            error_type: ErrorType::RuntimeError(name.clone()),
                        })
                    }
                } else {
                    Err(Error {
                        message: "Only instances have properties.".to_string(),
                        error_type: ErrorType::RuntimeError(name.clone()),
                    })
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_set_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Set {
                object,
                name,
                value,
            } => {
                // object.name = value
                let object = object.accept(self)?;
                if let Object::Instance(instance) = object {
                    let value = self.evaluate(value)?;
                    instance.borrow_mut().set(&name.lexeme, &value);
                    Ok(value)
                } else {
                    Err(Error {
                        message: "Only instances have fields.".to_string(),
                        error_type: ErrorType::RuntimeError(name.clone()),
                    })
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_index_set_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::IndexSet {
                object,
                index,
                index_end,
                value,
            } => {
                // TODO
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }
    fn visit_this_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::This { keyword } => Ok(self.look_up_variable(keyword)?),
            _ => unreachable!(),
        }
    }
    fn visit_super_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Super { keyword, method } => {
                let distance = self.locals.get(keyword);
                let super_class = self.look_up_variable(keyword)?;
                let object = self
                    .environment
                    .borrow()
                    .get_at(*distance.unwrap() - 1, "this")
                    .unwrap();

                if let Object::Class(super_class) = super_class {
                    if let Some(method) = super_class.borrow().get_method(&method.lexeme) {
                        Ok(Object::Callable(method.bind(object)))
                    } else {
                        Err(Error {
                            message: format!("Undefined property '{}'.", method.lexeme),
                            error_type: ErrorType::RuntimeError(method.clone()),
                        })
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
    fn visit_list_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::List { elements, .. } => {
                let mut list = List::new();
                for element in elements {
                    list.push(self.evaluate(element)?);
                }
                Ok(Object::List(Rc::new(RefCell::new(list))))
            }
            _ => unreachable!(),
        }
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ExprStmt { expression } => {
                self.evaluate(expression)?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", Interpreter::stringify(&value));
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::VarStmt { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Object::Nil,
                };
                self.environment.borrow_mut().define(&name.lexeme, value);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::BlockStmt { statements } => {
                // create a new scope
                let sub_env = Rc::new(RefCell::new(Environment::new(Some(
                    self.environment.clone(),
                ))));

                self.execute_block(statements, sub_env)
            }
            _ => unreachable!(),
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                if Interpreter::is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::WhileStmt { condition, body } => {
                while Interpreter::is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
                Ok(())
            }
            _ => unreachable!(),
        }
    }

    fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::FunStmt { name, params, body } => {
                let function = Object::Callable(Function::UserDefined {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                    is_initializer: false,
                });

                self.environment.borrow_mut().define(&name.lexeme, function);

                Ok(())
            }
            _ => unreachable!(),
        }
    }
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ReturnStmt { value, .. } => {
                let value = match value {
                    Some(expr) => self.evaluate(expr)?,
                    None => Object::Nil,
                };
                Err(Error {
                    message: String::from("Return statement"),
                    error_type: ErrorType::Return(value),
                })
            }
            _ => unreachable!(),
        }
    }
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ClassStmt {
                name,
                methods,
                super_class,
            } => {
                let mut super_class_ref: Option<ClassRef> = None;
                if let Some(super_class) = super_class {
                    let super_class_obj = self.evaluate(super_class)?;
                    if let Object::Class(super_class) = super_class_obj {
                        super_class_ref = Some(super_class);
                    } else {
                        return Err(Error {
                            message: "Superclass must be a class.".to_string(),
                            error_type: ErrorType::RuntimeError(name.clone()),
                        });
                    }
                }

                super_class_ref.as_ref().map(|super_class| -> Option<_> {
                    let sub_env = Rc::new(RefCell::new(Environment::new(Some(
                        self.environment.clone(),
                    ))));
                    self.environment = sub_env;

                    self.environment
                        .borrow_mut()
                        .define("super", Object::Class(super_class.clone()));
                    Some(())
                });
                let mut class_methods = HashMap::new();
                for method in methods {
                    match method {
                        Stmt::FunStmt { name, params, body } => {
                            let function = Function::UserDefined {
                                name: name.clone(),
                                params: params.clone(),
                                body: body.clone(),
                                closure: self.environment.clone(),
                                is_initializer: name.lexeme == "init",
                            };

                            class_methods.insert(name.lexeme.clone(), function);
                        }
                        _ => unreachable!(),
                    }
                }

                super_class_ref.as_ref().map(|_| -> Option<_> {
                    let previous = self
                        .environment
                        .borrow()
                        .enclosing
                        .as_ref()
                        .unwrap()
                        .clone();
                    self.environment = previous;
                    Some(())
                });

                let class_inner = Rc::new(RefCell::new(LoxClass::new(
                    name.lexeme.clone(),
                    class_methods,
                    super_class_ref,
                )));

                let class = Object::Class(class_inner);
                self.environment.borrow_mut().define(&name.lexeme, class);
                Ok(())
            }
            _ => unreachable!(),
        }
    }
}
