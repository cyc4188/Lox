use crate::Error;
use super::{Expr, Token};

pub mod stmt {
    use super::{Stmt, Error};
    pub trait Visitor<T> {
        fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
    }
}

/// statement      → exprStmt
///                | ifStmt ;
///                | printStmt ;
///                | blcok ;
///                | whileStmt ;
/// exprStmt       → expression ";" ;
/// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
/// printStmt      → "print" expression ";" ;
/// blcok          → "{" declaration* "}" ;
/// whileStmt      | "while" "(" expression ")" statement ;
#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt {
        expression: Expr,
    },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    PrintStmt {
        expression: Expr,
    },
    VarStmt {
        name: Token,
        initializer: Option<Expr>,
    },
    BlockStmt {
        statements: Vec<Stmt>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    }
}

impl Stmt {
    #[allow(unused_variables)]
    pub fn accept<T>(&self, visitor: &mut impl stmt::Visitor<T>) -> Result<T, Error> {
        match self {
            Stmt::ExprStmt { expression } => visitor.visit_expr_stmt(self),
            Stmt::IfStmt { condition, then_branch, else_branch } => visitor.visit_if_stmt(self),
            Stmt::PrintStmt { expression } => visitor.visit_print_stmt(self), 
            Stmt::VarStmt { name, initializer } => visitor.visit_var_stmt(self),
            Stmt::BlockStmt { statements } => visitor.visit_block_stmt(self),
            Stmt::WhileStmt { condition, body } => visitor.visit_while_stmt(self),
        }
    }
}