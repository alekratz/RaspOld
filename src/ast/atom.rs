use std::fmt;

#[derive(Clone, Debug)]
pub enum Atom {
    IntLit(i64),
    DubLit(f64),
    StrLit(String),
    Identifier(String),
}

pub fn atom_is_true(atom: &Atom) -> bool {
    match atom {
        &Atom::IntLit(i) => i != 0,
        &Atom::DubLit(f) => f != 0.0,
        &Atom::StrLit(ref s) => s.len() > 0,
        _ => panic!("Literals only"),
    }
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