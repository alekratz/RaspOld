mod ast;
mod rasp;
mod util;
mod internal;
mod interpreter;

extern crate lalrpop_util;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use util::RaspParseError;
use interpreter::{BytecodeGen, Interpreter};
use ast::visitor::MutVisitor;

macro_rules! printerrln(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    let mut error_occurred = false;
    for arg in env::args().skip(1) {
        let contents = if let Ok(mut fp) = File::open(&arg) {
            printerrln!("Opened {}", &arg);
            // read contents
            let mut contents = String::new();
            if fp.read_to_string(&mut contents).is_ok() {
                contents
            }
            else {
                error_occurred = true;
                printerrln!("Could not read {}", &arg);
                continue;
            }
        }
        else {
            error_occurred = true;
            printerrln!("No such file named {}", &arg);
            continue;
        };
        // eval
        let result = rasp::parse_CompileUnit(&contents);
        if let Err(err) = result {
            printerrln!("{}", RaspParseError::new(err, &contents, &arg));
            error_occurred = true;
            continue;
        }

        let expr_list = result.unwrap();
        let mut gen = BytecodeGen::new();
        for ast in expr_list {
            gen.visit_expression(&ast);
        }

        for warn in gen.warnings {
            printerrln!("WARN: {}", warn);
        }

        if gen.errors.len() == 0 {
            println!("Bytecode dump:");
            for b in &gen.bytecode {
                println!("{:?}", b);
            }
            // Interpret
            let mut interp = Interpreter::new();
            if let Err(err) = interp.interpret(&gen.bytecode) {
                printerrln!("ERR:  {}", err);
            }
        }
        else {
            for err in gen.errors {
                printerrln!("ERR:  {}", err);
            }
        }
    }

    if error_occurred {
        printerrln!("Error occurred; aborting");
        std::process::exit(1);
    }
}
