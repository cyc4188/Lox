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
        fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
    }
}

/// statement      → exprStmt
///                | ifStmt ;
///                | printStmt ;
///                | blcok ;
///                | whileStmt ;
///                | forStmt ;
/// exprStmt       → expression ";" ;
/// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
/// printStmt      → "print" expression ";" ;
/// blcok          → "{" declaration* "}" ;
/// whileStmt      | "while" "(" expression ")" statement ;
/// forStmt        | "for" "(" ( varDecl | exprStmt | ";" )
///                         expression? ";"
///                         expression? ")" statement ; 
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
    },
    FunStmt {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    }
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl stmt::Visitor<T>) -> Result<T, Error> {
        match self {
            Stmt::ExprStmt { .. } => visitor.visit_expr_stmt(self),
            Stmt::IfStmt { .. } => visitor.visit_if_stmt(self),
            Stmt::PrintStmt { .. } => visitor.visit_print_stmt(self), 
            Stmt::VarStmt { .. } => visitor.visit_var_stmt(self),
            Stmt::BlockStmt {.. } => visitor.visit_block_stmt(self),
            Stmt::WhileStmt { .. } => visitor.visit_while_stmt(self),
            Stmt::FunStmt { .. } => visitor.visit_func_stmt(self),
        }
    }
}