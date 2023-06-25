use std::fmt::Display;

use super::{Expr, Token};
use crate::{AstPrinter, Error};

pub mod stmt {
    use super::{Error, Stmt};
    pub trait Visitor<T> {
        fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
        fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<T, Error>;
    }
}

/// statement      → exprStmt
///                | ifStmt ;
///                | printStmt ;
///                | blcok ;
///                | whileStmt ;
///                | forStmt ;
///                | returnStmt
/// exprStmt       → expression ";" ;
/// ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
/// printStmt      → "print" expression ";" ;
/// blcok          → "{" declaration* "}" ;
/// whileStmt      | "while" "(" expression ")" statement ;
/// forStmt        | "for" "(" ( varDecl | exprStmt | ";" )
///                         expression? ";"
///                         expression? ")" statement ;
/// returnStmt     | "return" expression? ";" ;
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
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
    ClassStmt {
        name: Token,
        super_class: Option<Expr>,
        methods: Vec<Stmt>,
    },
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl stmt::Visitor<T>) -> Result<T, Error> {
        match self {
            Stmt::ExprStmt { .. } => visitor.visit_expr_stmt(self),
            Stmt::IfStmt { .. } => visitor.visit_if_stmt(self),
            Stmt::PrintStmt { .. } => visitor.visit_print_stmt(self),
            Stmt::VarStmt { .. } => visitor.visit_var_stmt(self),
            Stmt::BlockStmt { .. } => visitor.visit_block_stmt(self),
            Stmt::WhileStmt { .. } => visitor.visit_while_stmt(self),
            Stmt::FunStmt { .. } => visitor.visit_func_stmt(self),
            Stmt::ReturnStmt { .. } => visitor.visit_return_stmt(self),
            Stmt::ClassStmt { .. } => visitor.visit_class_stmt(self),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printer = AstPrinter::new();
        write!(f, "{}", self.accept(&mut printer).unwrap())
    }
}

impl stmt::Visitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::BlockStmt { statements } => {
                let mut s = String::new();
                s.push_str("block: {\n");
                for stmt in statements {
                    s.push_str(stmt.accept(self)?.as_str());
                    s.push('\n');
                }
                s.push('}');
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::ClassStmt { name, methods, .. } => {
                let mut s = String::new();
                s.push_str("class: ");
                s.push_str(name.lexeme.as_str());
                s.push_str(" {\n");
                for stmt in methods {
                    s.push_str(stmt.accept(self)?.as_str());
                    s.push('\n');
                }
                s.push('}');
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_expr_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::ExprStmt { expression } => Ok(expression.accept(self)?.as_str().to_string()),
            _ => unreachable!(),
        }
    }
    fn visit_func_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::FunStmt { name, params, body } => {
                let mut s = String::new();
                s.push_str("fun: ");
                s.push_str(name.lexeme.as_str());
                s.push_str(" (");
                for param in params {
                    s.push_str(param.lexeme.as_str());
                    s.push_str(", ");
                }
                s.push_str(") {\n");
                for stmt in body {
                    s.push_str(stmt.accept(self)?.as_str());
                    s.push('\n');
                }
                s.push('}');
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut s = String::new();
                s.push_str("if: ");
                s.push_str(condition.accept(self)?.as_str());
                s.push_str(" then: ");
                s.push_str(then_branch.accept(self)?.as_str());
                if let Some(else_branch) = else_branch {
                    s.push_str(" else: ");
                    s.push_str(else_branch.accept(self)?.as_str());
                }
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::PrintStmt { expression } => {
                let mut s = String::new();
                s.push_str("print: ");
                s.push_str(expression.accept(self)?.as_str());
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::ReturnStmt { keyword, value } => {
                let mut s = String::new();
                s.push_str("return: ");
                s.push_str(keyword.lexeme.as_str());
                if let Some(value) = value {
                    s.push(' ');
                    s.push_str(value.accept(self)?.as_str());
                }
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::VarStmt { name, initializer } => {
                let mut s = String::new();
                s.push_str("var: ");
                s.push_str(name.lexeme.as_str());
                if let Some(initializer) = initializer {
                    s.push_str(" = ");
                    s.push_str(initializer.accept(self)?.as_str());
                }
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<String, Error> {
        match stmt {
            Stmt::WhileStmt { condition, body } => {
                let mut s = String::new();
                s.push_str("while: ");
                s.push_str(condition.accept(self)?.as_str());
                s.push_str(" body: ");
                s.push_str(body.accept(self)?.as_str());
                Ok(s)
            }
            _ => unreachable!(),
        }
    }
}
