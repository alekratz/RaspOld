use std::collections::HashMap;
use ast::*;
use ast::visitor::MutVisitor;
use internal::*;
use interpreter::{BytecodeGen, VarDecls};

pub struct Interpreter {
    stack: Vec<Atom>,
    functions: HashMap<String, Function>,
    builtin_functions: HashMap<&'static str, BuiltinFunction>,
    function_code: HashMap<String, Vec<Bytecode>>,
    generator: BytecodeGen,
    decl_stack: Vec<VarDecls>, /* This one holds variables declared in functions */
    global_decls: VarDecls, /* A list of variables defined in the global scope */
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack: vec![],
            functions: HashMap::new(),
            builtin_functions: make_builtin_functions(),
            function_code: HashMap::new(),
            generator: BytecodeGen::new(&vec![]),
            decl_stack: vec![HashMap::new()],
            global_decls: HashMap::new(),
        }
    }

    fn set_var(&mut self, label: &str, value: Atom) {
        if self.decl_stack.len() == 0 {
            self.global_decls.insert(label.to_string(), value);
        }
        else {
            let mut var_decls = self.decl_stack
                .last_mut()
                .unwrap();
            var_decls.insert(label.to_string(), value);
        }
    }

    fn get_var(&self, label: &str) -> Option<&Atom> {
        let var_decls = self.decl_stack
            .last()
            .unwrap();
        match var_decls.get(label) {
            Some(ref atom) => Some(atom),
            None => self.global_decls.get(label)
        }
    }

    /**
     * Executes a user-defined function based on its name
     */
    fn user_defined_function(&mut self, fname: &str) -> Result<(), String> {
        // Handle user-defined function
        let code = match self.function_code.get(fname) {
            Some(code) => code.clone(),
            None => match self.functions.get(fname) {
                Some(fun) => {
                    self.generator = BytecodeGen::new(&fun.args);
                    for body in &fun.body {
                        self.generator.visit_expression(&body);
                    }
                    self.generator.bytecode.clone()
                },
                None => panic!("Function {} not found", fname), 
            }
        };
        // Insert the function call if it doesn't already exist in the cache
        self.function_code
            .entry(fname.to_string())
            .or_insert(code.clone());
        // Add a new var decl list to the decl stack
        let locals = VarDecls::new();
        self.decl_stack.push(locals);
        // one clone of a string, and at least one and up to two clones of the code
        if let Err(err) = self.interpret(&code) {
            Err(err)
        }
        else {
            self.decl_stack.pop();
            Ok(())
        }
    }

    /**
     * Executes a builtin function
     */
    fn builtin_function(&mut self, fname: &str) -> Result<(), String> {
        // Handle builtin function
        let argcount = self.builtin_functions
            .get(fname)
            .unwrap()
            .args
            .len();
        let mut args = Vec::new();
        for _ in 0 .. argcount {
            match self.pop() {
                Ok(atom) => args.push(atom),
                Err(err) => return Err(err),
            }
        }
        let locals = self.decl_stack
            .iter()
            .last();
        let context = BuiltinContext { args: args, global_decls: &self.global_decls, var_decls: locals};
        let func = self.builtin_functions
            .get(fname)
            .unwrap();
        let result = (func.fun)(context);
        match result {
            Ok(atom) => {
                self.stack.push(atom);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    /**
     * Handles a call instruction
     */
    fn handle_call(&mut self, fname: &str) -> Result<(), String> {
        /*
        println!("  Function call stack ({}):", fname);
        for s in &self.stack {
            println!("      {:?}", s);
        }
        println!("  Var decls");
        for s in &self.decl_stack {
            println!("    {:?}", s);
        }
        */
        if self.functions.contains_key(fname) {
            self.user_defined_function(fname)
        }
        else if self.builtin_functions.contains_key(fname) {
            self.builtin_function(fname)
        }
        else { 
            Err(format!("Function {} not found", fname))
        }
    }

    pub fn interpret(&mut self, bytecode: &Vec<Bytecode>) -> Result<(), String> {
        let mut index = 0;
        if bytecode.len() == 0 {
            return Ok(())
        }
        /*
        println!(" ---- BYTECODE DUMP ----");
        for b in bytecode {
            println!("{:?}", b);
        }
        */
        // Get the label locations
        let labels = Interpreter::get_labels(bytecode);
        loop {
            let ref code = bytecode[index];
            //println!("{:?}", code);

            match code {
                &Bytecode::Nop => {}, // skip it
                &Bytecode::Call(ref fname) => if let Err(err) = self.handle_call(fname) {
                    return Err(err);
                },
                &Bytecode::Push(ref v) => self.stack.push(v.clone()),
                &Bytecode::FunDef(ref func) => { self.functions.insert(func.name.to_string(), func.clone()); },
                &Bytecode::Label(ref lnum) => { /* ignore */ },
                &Bytecode::Jump(ref lnum) => {
                    assert!(labels.contains_key(lnum), "Unknown label found");
                    index = labels[lnum];
                    continue;
                },
                &Bytecode::JumpTrue(ref lnum) => {
                    assert!(labels.contains_key(lnum), "Unknown label found");
                    // Pop off the top item from the stack
                    let result = self.pop();
                    if let Err(err) = result {
                        return Err(err);
                    }

                    let atom_val = result.unwrap();
                    if let Atom::Identifier(_) = atom_val {
                        return Err("Identifiers are invalid for truth values".to_string());
                    }
                    else if atom_val.is_true() {
                        index = labels[lnum];
                    }
                },
                &Bytecode::Pop(ref into) => if let &Some(ref label) = into {
                        match self.pop() {
                            Ok(atom) => self.set_var(label, atom),
                            Err(err) => return Err(err), 
                        }
                    }
                    else {
                        match self.pop() {
                            Ok(_) => {},
                            Err(err) => return Err(err), 
                        }
                    }
            }
            index += 1;
            if index >= bytecode.len() {
                break;
            }
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