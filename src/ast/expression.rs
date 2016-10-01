use ast::{Atom,Function,IfElse};

#[derive(Clone, Debug)]
pub enum Expression {
    Atom(Atom),
    Children(Vec<Expression>),
    Unit,
    FunDef(Function),
    IfElse(Box<IfElse>),
}
