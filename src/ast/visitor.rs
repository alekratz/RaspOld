use ast::expression::*;
use ast::atom::*;

pub trait Visitor<R> {
    fn visit_expression(&self, expr: &Expression) -> R;
    fn visit_atom(&self, expr: &Atom) -> R;
    fn visit_ifelse(&self, expr: &IfElse) -> R;
}

pub trait MutVisitor<R> {
    fn visit_expression(&mut self, expr: &Expression) -> R;
    fn visit_atom(&mut self, expr: &Atom) -> R;
    fn visit_ifelse(&mut self, expr: &IfElse) -> R;
}
