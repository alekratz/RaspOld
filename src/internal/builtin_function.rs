use ast::Atom;
use std::collections::HashMap;
use interpreter::VarDecls;

pub type BuiltinImp = fn(BuiltinContext) -> Result<Atom, String>; 

/**
 * Builtin functions
 */
pub struct BuiltinFunction {
    pub name: &'static str,
    pub args: Vec<&'static str>,
    pub fun: BuiltinImp,
}

/**
 * Context for builtin functions to use
 */
pub struct BuiltinContext<'a> {
    pub args: Vec<Atom>,
    pub global_decls: &'a VarDecls,
    pub var_decls: Option<&'a VarDecls>,
}

impl<'a> BuiltinContext<'a> {
    fn get_var(&self, label: &str) -> Option<&Atom> {
        if let Some(decls) = self.var_decls {
            match decls.get(label) {
                Some(ref atom) => Some(atom),
                None => self.global_decls.get(label), 
            }
        }
        else {
            self.global_decls.get(label)
        }
    }
}

/**
 * Creates the list of builtin functions.
 */
pub fn make_builtin_functions() -> HashMap<&'static str, BuiltinFunction> {
    let mut the_map = HashMap::new();

    the_map.insert("&print", BuiltinFunction { name: "&print", args: vec!["fmt"], fun: print_builtin });
    the_map.insert("==", BuiltinFunction { name: "==", args: vec!["lhs", "rhs"], fun: equals_builtin });
    the_map.insert("-", BuiltinFunction { name: "-", args: vec!["lhs", "rhs"], fun: minus_builtin });
    the_map.insert("*", BuiltinFunction { name: "*", args: vec!["lhs", "rhs"], fun: times_builtin });
    the_map
}

fn print_builtin(context: BuiltinContext) -> Result<Atom, String> {
    let args = context.args;
    if args.len() == 0 {
        println!("");
        Ok(Atom::IntLit(0))
    }
    else {
        let mut count = 0;
        print!("{}", args[0]);
        for a in args.iter().skip(1) {
            print!(" {}", a);
            count += 1;
        }
        println!("");
        Ok(Atom::IntLit(count))
    }
}

fn unwrap_arg<'a>(context: &'a BuiltinContext, n: usize) -> Result<&'a Atom, String> {
    let ref args = context.args;
    let ref var = args[n];
    if let &Atom::Identifier(ref s) = var {
        if let Some(atom) = context.get_var(s) {
            Ok(atom)
        }
        else {
            Err(format!("Undefined variable: {}", s))
        }
    }
    else {
        Ok(var)
    }
}

fn equals_builtin(context: BuiltinContext) -> Result<Atom, String> {
    let ref args = context.args;
    if args.len() != 2 {
        Err(format!("Invalid number of arguments for `=='; got {} but expected exactly 2", args.len()))
    }
    else {
        let lhs_result = unwrap_arg(&context, 1);
        let rhs_result = unwrap_arg(&context, 0);
        
        if lhs_result.is_err() {
            Err(lhs_result.unwrap_err())
        }
        else if rhs_result.is_err() {
            Err(rhs_result.unwrap_err())
        }
        else {
            let lhs = lhs_result.unwrap();
            let rhs = rhs_result.unwrap();
            Ok(lhs.equals(rhs))
        }
    }
}

fn minus_builtin(context: BuiltinContext) -> Result<Atom, String> {
    // This expects either one or two arguments
    let ref args = context.args;
    if args.len() == 1 {
        match args[0] {
            Atom::Identifier(ref s) => {
                let result = context.get_var(s);
                if result.is_some() {
                    result.unwrap().neg()
                }
                else {
                    Err(format!("Undefined variable: {}", s))
                }
            }
            _ => Ok(Atom::IntLit(0i64))
        }
    }
    else if args.len() == 2 {
        // do this in reverse because stacks
        let lhs_result = unwrap_arg(&context, 1);
        let rhs_result = unwrap_arg(&context, 0);
        // TODO : look up error chain
        if lhs_result.is_err() {
            Err(lhs_result.unwrap_err())
        }
        else if rhs_result.is_err() {
            Err(rhs_result.unwrap_err())
        }
        else {
            let lhs = lhs_result.unwrap();
            let rhs = rhs_result.unwrap();
            lhs.minus(rhs)
        }
    }
    else {
        Err(format!("Invalid number of arguments for `-'; got {} but expected either 1 or 2", args.len()))
    }
}

fn times_builtin(context: BuiltinContext) -> Result<Atom, String> {
    // This expects either one or two arguments
    let ref args = context.args;
    if args.len() == 2 {
        // do this in reverse because stacks
        let lhs_result = unwrap_arg(&context, 1);
        let rhs_result = unwrap_arg(&context, 0);
        
        if lhs_result.is_err() {
            Err(lhs_result.unwrap_err())
        }
        else if rhs_result.is_err() {
            Err(rhs_result.unwrap_err())
        }
        else {
            let lhs = lhs_result.unwrap();
            let rhs = rhs_result.unwrap();
            lhs.times(rhs)
        }
    }
    else {
        Err(format!("Invalid number of arguments for `-'; got {} but expected either 1 or 2", args.len()))
    }
}