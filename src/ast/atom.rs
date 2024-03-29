use std::fmt;

#[derive(Clone, Debug)]
pub enum Atom {
    IntLit(i64),
    DubLit(f64),
    BoolLit(bool),
    StrLit(String),
    Identifier(String),
}

impl Atom {
    pub fn is_true(&self) -> bool {
        match self {
            &Atom::IntLit(i) => i != 0,
            &Atom::DubLit(f) => f != 0.0,
            &Atom::BoolLit(b) => b,
            &Atom::StrLit(ref s) => s.len() > 0,
            _ => panic!("Literals only"),
        }
    }

    pub fn equals(&self, other: &Atom) -> Atom {
        match self {
            &Atom::IntLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Atom::BoolLit(*lhs == *rhs)
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Atom::BoolLit(*lhs as f64 == *rhs)
                }
                else if let &Atom::BoolLit(ref rhs) = other {
                    Atom::BoolLit((*lhs == 1 && *rhs) || (*lhs == 0 && !*rhs))
                }
                else {
                    Atom::BoolLit(false)
                },
            &Atom::DubLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Atom::BoolLit(*lhs == *rhs as f64)
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Atom::BoolLit(*lhs == *rhs)
                }
                else if let &Atom::BoolLit(ref rhs) = other {
                    Atom::BoolLit((*lhs == 1.0 && *rhs) || (*lhs == 0.0 && !*rhs))
                }
                else {
                    Atom::BoolLit(false)
                },
            &Atom::BoolLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Atom::BoolLit((*lhs && *rhs == 1) || (!*lhs && *rhs == 0))
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Atom::BoolLit((*lhs && *rhs == 1.0) || (!*lhs && *rhs == 0.0))
                }
                else if let &Atom::BoolLit(ref rhs) = other {
                    Atom::BoolLit(*lhs == *rhs)
                }
                else {
                    Atom::BoolLit(false)
                },
            &Atom::StrLit(ref lhs) => if let &Atom::StrLit(ref rhs) = other {
                    Atom::BoolLit(*lhs == *rhs)
                } 
                else {
                    Atom::BoolLit(false)
                },
            _ => Atom::BoolLit(false)
        }
    }

    pub fn neg(&self) -> Result<Atom, String> {
        match self {
            &Atom::IntLit(ref i) => Ok(Atom::IntLit(-i)),
            &Atom::DubLit(ref i) => Ok(Atom::DubLit(-i)),
            // TODO : make this more specific
            _ => Err("Invalid operation on string or identifier".to_string())            
        }
    }

    pub fn minus(&self, other: &Atom) -> Result<Atom, String> {
        match self {
            &Atom::IntLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Ok(Atom::IntLit(*lhs - *rhs))
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs as f64 - *rhs))
                }
                else {
                    Err("Invalid operands".to_string())
                },
            &Atom::DubLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs - *rhs as f64))
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs - *rhs))
                }
                else {
                    Err("Invalid operands for -".to_string())
                },
            _ => Err("Invalid operands for -".to_string())
        }
    }

    pub fn times(&self, other: &Atom) -> Result<Atom, String> {
        match self {
            &Atom::IntLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Ok(Atom::IntLit(*lhs * *rhs))
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs as f64 * *rhs))
                }
                else {
                    Err("Invalid operands".to_string())
                },
            &Atom::DubLit(ref lhs) => if let &Atom::IntLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs * *rhs as f64))
                }
                else if let &Atom::DubLit(ref rhs) = other {
                    Ok(Atom::DubLit(*lhs * *rhs))
                }
                else {
                    Err("Invalid operands for *".to_string())
                },
            _ => Err("Invalid operands for *".to_string())
        }
    }
}


impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Atom::IntLit(i) => write!(f, "{}", i),
            &Atom::DubLit(d) => write!(f, "{}", d),
            &Atom::BoolLit(b) => write!(f, "{}", b),
            &Atom::StrLit(ref s) => write!(f, "{}", s),
            &Atom::Identifier(ref n) => write!(f, "{}", n),
        }
    }
}