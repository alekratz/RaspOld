use internal::Function;
use ast::{Atom,Expression};

#[derive(Clone, Debug)]
pub enum Bytecode {
    Nop,
    Call(String),           // Calls a function, using the current working stack for the args
    Push(Atom),             // Pushes a value to the working stack
    //Pop(Option<String>),  // Pops a value off of the stack, optionally into an identifier (or register?)
    FunDef(Function),       // Defines a function that will be compiled upon its first use
    Label(u64),             // Defines a location that can be jumped to
    Jump(u64),              // Jumps to a label that's been defined
    JumpEq(u64, Expression, Expression),
                            // Jumps to a label based on the equality of two expressions
    JumpGt(u64, Expression, Expression),
                            // Jumps to a label based on whether the first expression is greater than the second
    JumpLt(u64, Expression, Expression),
                            // Jumps to a label based on whether the first expression is less than the second
}
