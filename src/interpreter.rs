use super::*;

pub struct Interpreter;

impl Interpreter {

}

impl Visitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            _ => Err(Error::new("Expected literal expression", ErrorType::SyntaxError)),
        }
    }
    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        todo!()
    }
    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        todo!()
    }
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, Error> {
        todo!()
    }
}