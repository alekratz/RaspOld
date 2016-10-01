use std::collections::HashMap;
use ast::visitor::Visitor;
use ast::{Expression, Atom};
use internal::Function;
use interpreter::Errorable; 

/**
 * Collects all defined functions
 */
pub struct FunctionCollector<'f> {
    errors: Vec<String>,
    warnings: Vec<String>,
    functions: HashMap<String, Function<'f>>,
}

impl<'f> FunctionCollector<'f> {
    pub fn collect(&mut self, root_expr: &Expression) {
        self.visit_expression(root_expr)
    }
}

impl<'f> Visitor<()> for FunctionCollector<'f> {
    fn visit_expression(&self, expr: &Expression) {
    }

    fn visit_atom(&self, expr: &Atom) {
    }
}

impl<'f> Errorable<String> for FunctionCollector<'f> {
    fn error(&mut self, err: String) {
        self.errors.push(err);
    }

    fn warn(&mut self, warn: String) {
        self.warnings.push(warn);
    }
}
