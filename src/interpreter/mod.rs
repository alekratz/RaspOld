//mod collector;
//pub use interpreter::collector::FunctionCollector;

mod bytecode_gen;
mod interpreter;
pub use interpreter::bytecode_gen::BytecodeGen;
pub use interpreter::interpreter::Interpreter;

