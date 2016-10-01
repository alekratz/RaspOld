use ast::visitor::MutVisitor;
use ast::{Expression,Atom,IfElse};
use internal::Bytecode;
use std::ops::Deref;

pub struct BytecodeGen {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub bytecode: Vec<Bytecode>,
    label_count: u64,
}

impl BytecodeGen {
    pub fn new() -> BytecodeGen {
        BytecodeGen { 
            errors: vec![],
            warnings: vec![],
            bytecode: vec![],
            label_count: 0u64,
        }
    }

    pub fn next_label(&mut self) -> u64 {
        self.label_count += 1;
        self.label_count - 1
    }

    pub fn was_err(&self) -> bool {
        self.errors.len() > 0
    }

    pub fn reset(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.bytecode.clear();
    }

    fn err(&mut self, msg: String) {
        self.errors.push(msg);
    }
    fn warn(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    fn handle_children(&mut self, children: &Vec<Expression>) {
        assert!(children.len() > 0);
        let ref first = children[0];
        let mut fun_name: Option<String> = None;

        match first {
            &Expression::Children(ref c) => if children.len() > 1 && c.len() > 1 {
                self.err("Dynamic function dispatch not yet supported".to_string());
                return;
            }
            else { self.handle_children(c) },
            // Match the atom as a function with special cases
            // * If there's more than 1 argument, then it's treated as a function.
            //   * This will cause ints and floats to fail as the first argument
            //   * Strings will be resolved as identifiers
            // * If there's only 1 argument, then it's treated as a value.
            //   * Identifiers are the only values that break this rule. They are treated as function calls.
            &Expression::Atom(ref a) => match a {
                &Atom::IntLit(_) => 
                    if children.len() > 1 { self.err("Invalid function call with int literal".to_string()) }
                    else { self.visit_atom(a); },
                &Atom::DubLit(_) => 
                    if children.len() > 1 { self.err("Invalid function call with float literal".to_string()) }
                    else { self.visit_atom(a); },
                &Atom::StrLit(ref s) =>
                    if children.len() > 1 { fun_name = Some(s.clone()); } // we can use strings to call functions 
                    else { self.visit_atom(a); },
                &Atom::Identifier(ref i) => fun_name = Some(i.clone()), // function name every time
            },
            &Expression::Unit => self.bytecode.push(Bytecode::Nop),
            &Expression::FunDef(_) => self.err("Function definitions are not yet allowed below top level.".to_string()),
            &Expression::IfElse(ref b) => self.visit_ifelse(b.deref())
        }

        if let Some(fun_str) = fun_name {
            // function call
            for child in children.iter().skip(1) {
                match child {
                    &Expression::Children(ref c) => self.handle_children(c),
                    &Expression::Atom(ref a) => self.visit_atom(a),
                    &Expression::Unit => self.bytecode.push(Bytecode::Nop),
                    &Expression::FunDef(_) => self.err("Function definitions are not yet allowed below top level.".to_string()),
                    &Expression::IfElse(ref b) => self.visit_ifelse(b.deref())
                }
            }
            self.bytecode.push(Bytecode::Call(fun_str));
        }
    }
}

impl MutVisitor<()> for BytecodeGen {
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            &Expression::Atom(ref a) => self.visit_atom(a),
            &Expression::Children(ref c) => self.handle_children(c),
            &Expression::Unit => self.bytecode.push(Bytecode::Nop),
            &Expression::FunDef(ref f) => self.bytecode.push(Bytecode::FunDef(f.clone())),
            &Expression::IfElse(ref b) => self.visit_ifelse(b.deref())
        }
    }

    fn visit_atom(&mut self, atom: &Atom) {
        self.bytecode.push(Bytecode::Push(atom.clone()));
    }

    fn visit_ifelse(&mut self, ifelse: &IfElse) {
        self.visit_expression(&ifelse.condition);
        let truelabel = self.next_label();
        self.bytecode.push(Bytecode::JumpTrue(truelabel));
        if let Some(ref if_false) = ifelse.if_false {
            self.visit_expression(if_false);
        }
        let donelabel = self.next_label();
        self.bytecode.push(Bytecode::Jump(donelabel));
        self.bytecode.push(Bytecode::Label(truelabel));
        self.visit_expression(&ifelse.if_true);
        self.bytecode.push(Bytecode::Label(donelabel));
    }
}
