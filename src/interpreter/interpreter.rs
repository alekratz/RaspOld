use std::collections::HashMap;
use ast::{Atom,atom_is_true};
use ast::visitor::MutVisitor;
use internal::{Function,Bytecode};
use interpreter::BytecodeGen;

pub struct Interpreter {
    stack: Vec<Atom>,
    functions: HashMap<String, Function>,
    //builtin_functions: HashMap<String, BuiltinFunction>,
    function_code: HashMap<String, Vec<Bytecode>>,
    generator: BytecodeGen,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack: vec![],
            functions: HashMap::new(),
            function_code: HashMap::new(),
            generator: BytecodeGen::new(),
        }
    }

    pub fn interpret(&mut self, bytecode: &Vec<Bytecode>) -> Result<(), String> {
        let mut index = 0;
        if bytecode.len() == 0 {
            return Ok(())
        }
        // Get the label locations
        let labels = Interpreter::get_labels(bytecode);
        loop {
            let ref code = bytecode[index];

            match code {
                &Bytecode::Nop => {}, // skip it
                &Bytecode::Call(ref fname) => {
                    if !self.functions.contains_key(fname) {
                        return Err(format!("Function {} not found", fname));
                    }

                    let code = match self.function_code.get(fname) {
                        Some(code) => code.clone(),
                        None => match self.functions.get(fname) {
                            Some(fun) => {
                                self.generator = BytecodeGen::new();
                                for body in &fun.body {
                                    self.generator.visit_expression(&body);
                                }
                                self.generator.bytecode.clone()
                            },
                            None => panic!("Function {} not found", fname), 
                        }
                    };
                    self.function_code.entry(fname.to_string()).or_insert(code.clone());
                    // one clone of a string, and at least one and up to two clones of the code
                    if let Err(err) = self.interpret(&code) {
                        return Err(err);
                    }
                },
                &Bytecode::Push(ref v) => {
                    self.stack.push(v.clone());
                },
                // Bytecode::Pop(Option<String>),
                &Bytecode::FunDef(ref func) => {
                    self.functions.insert(func.name.to_string(), func.clone());
                },
                &Bytecode::Label(ref lnum) => { /* ignore */ },
                &Bytecode::Jump(ref lnum) => { 
                    assert!(labels.contains_key(lnum), "Unknown label found");
                    index = labels[lnum];
                    continue; // don't allow the += 1
                },
                &Bytecode::JumpTrue(ref lnum) => { 
                    assert!(labels.contains_key(lnum), "Unknown label found");
                    // Pop off the top item from the stack
                    let result = self.pop();
                    if let Err(err) = result {
                        return Err(err);
                    }
                    else {
                        let atom_val = result.unwrap();
                        if let Atom::Identifier(_) = atom_val {
                            return Err("Identifiers are invalid for truth values".to_string());
                        }
                        else if atom_is_true(&atom_val) {
                            index = labels[lnum];
                        }
                    }
                },
            }
            index += 1;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Atom, String> {
        if self.stack.len() == 0 {
            Err("Stack was empty but attempted to pop an item off".to_string())
        }
        else {
            Ok(self.stack
                .pop()
                .unwrap())
        }
    }

    fn get_labels(bytecode: &Vec<Bytecode>) -> HashMap<u64, usize> {
        let mut the_map = HashMap::new();
        let mut index = 0usize;
        for b in bytecode {
            match b {
                &Bytecode::Label(ref lnum) => { the_map.insert(*lnum, index); },
                _ => {  }
            }
            index += 1;
        }
        the_map
    }
}