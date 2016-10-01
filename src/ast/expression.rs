use ast::Atom;
use internal::Function;

#[derive(Clone, Debug)]
pub enum Expression {
    Atom(Atom),
    Children(Vec<Expression>),
    Unit,
    FunDef(Function),
    IfElse(Box<IfElse>),
}

#[derive(Clone, Debug)]
pub struct IfElse { 
    pub condition: Expression, 
    pub if_true: Expression, 
    pub if_false: Option<Expression>
}
