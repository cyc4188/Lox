use std::collections::HashMap;

use super::*;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    fun_depth: usize,
    pub has_error: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            fun_depth: 0,
            has_error: false,
        }
    }

    fn begin_fun(&mut self) {
        self.fun_depth += 1;
        self.begin_scope();
    }

    fn end_fun(&mut self) {
        self.fun_depth -= 1;
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        stmt.accept(self)
    }

    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        expr.accept(self)
    }

    fn declare(&mut self, name: &Token) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                parse_error(name, "Variable with this name already declared in this scope.");
                self.has_error = true;
            }
            scope.insert(name.lexeme.clone(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
        Ok(())
    }

    fn resolve_local(&mut self, _expr: &Expr, name: &Token) -> Result<(), Error> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
                return Ok(());
            }
        }
        Ok(())
        // not found
        // we assume it a global variable
    }
}


impl<'a> expr::Visitor<()> for Resolver<'a> {
    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last_mut() {
                    if scope.get(&name.lexeme) == Some(&false) {
                        parse_error(name, "Cannot read local variable in its own initializer.");
                        self.has_error = true;
                    }
                }
                self.resolve_local(expr, name)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(value)?;
                self.resolve_local(expr, name)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Binary { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_call_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Call { callee, arguments , ..} => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Grouping { expression } => {
                self.resolve_expr(expression)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_literal_expr(&mut self, _value: &Literal) -> Result<(), Error> {
        Ok(())
    }
    fn visit_logic_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Logical { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<(), Error> {
        match expr {
            Expr::Unary { right, .. } => {
                self.resolve_expr(right)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
}

impl<'a> stmt::Visitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::BlockStmt { statements } => {
                self.begin_scope();
                self.resolve_stmts(statements)?;
                self.end_scope();
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::VarStmt { name, initializer } => {
                self.declare(name)?;
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer)?;
                }
                self.define(name)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        trace!("Visiting function statement");
        match stmt {
            Stmt::FunStmt { name, params, body } => {
                self.declare(name)?;
                self.define(name)?;

                self.begin_fun();
                for param in params {
                    self.declare(param)?;
                    self.define(param)?;
                }
                self.resolve_stmts(body)?;
                self.end_fun();

                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ExprStmt { expression } => {
                self.resolve_expr(expression)?;
                Ok(())
            }
            _ => unreachable!() 
        }
    }
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::IfStmt { condition, then_branch, else_branch } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch)?;
                }
                Ok(())
            }
            _ => unreachable!() 
        }
    }
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::PrintStmt { expression } => {
                self.resolve_expr(expression)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ReturnStmt { value , keyword} => {
                if self.fun_depth == 0 {
                    parse_error(keyword, "Cannot return from top-level code.");
                    self.has_error = true;
                }
                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)?;
                Ok(())
            }
            _ => unreachable!()
        }
    }
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::ClassStmt { name, methods } => {
                self.declare(name)?;
                self.define(name)?;
                // TODO: Check methods
                Ok(())
            }
            _ => unreachable!()
        }
    }
}