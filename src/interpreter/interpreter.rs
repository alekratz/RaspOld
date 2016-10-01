use std::collections::HashMap;
use ast::Atom;
use ast::visitor::MutVisitor;
use internal::{Function,Bytecode};
use interpreter::BytecodeGen;

pub struct Interpreter {
    stack: Vec<Atom>,
    functions: HashMap<String, Function>,
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
        loop {
            let ref code = bytecode[index];

            match code {
                &Bytecode::Nop => {}, // skip it
                &Bytecode::Call(ref fname) => {
                    if !self.functions.contains_key(fname) {
                        println!("!!!!! Error: dumping bytecode !!!!!");
                        for b in bytecode {
                            println!("{:?}", b);
                        }
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
            }
            index += 1;
        }
        Ok(())
    }
}