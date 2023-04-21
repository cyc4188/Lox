use crate::Error;

use super::Expr;
use std::fmt;

pub mod stmt {
    use super::{Stmt, Error};
    pub trait Visitor<T> {
        fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
    }
}

// statement      → exprStmt
//                | printStmt ;
// exprStmt       → expression ";" ;
// printStmt      → "print" expression ";" ;
pub enum Stmt {
    ExprStmt {
        expression: Expr,
    },
    PrintStmt {
        expression: Expr,
    },
}

impl Stmt {
    #[allow(unused_variables)]
    pub fn accpet<T>(&self, visitor: &mut impl stmt::Visitor<T>) -> Result<T, Error> {
        match self {
            Stmt::ExprStmt { expression } => visitor.visit_expr_stmt(self),
            Stmt::PrintStmt { expression } => visitor.visit_print_stmt(self), 
        }
    }
}