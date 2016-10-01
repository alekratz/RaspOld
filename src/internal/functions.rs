use ast::Expression;

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Expression>,
}

impl Function {
    pub fn new(name: String, args: Vec<String>, body: Vec<Expression>) -> Function {
        Function {
            name: name,
            args: args,
            body: body,
        }
    }
}
