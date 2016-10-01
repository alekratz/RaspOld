use ast::Expression;

#[derive(Clone, Debug)]
pub struct IfElse { 
    pub condition: Expression, 
    pub if_true: Expression, 
    pub if_false: Option<Expression>
}
