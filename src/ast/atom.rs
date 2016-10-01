use std::fmt;

#[derive(Clone, Debug)]
pub enum Atom {
    IntLit(i64),
    DubLit(f64),
    StrLit(String),
    Identifier(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Atom::IntLit(i) => write!(f, "{}", i),
            &Atom::DubLit(d) => write!(f, "{}", d),
            &Atom::StrLit(ref s) => write!(f, "{}", s),
            &Atom::Identifier(ref n) => write!(f, "{}", n),
        }
    }
}