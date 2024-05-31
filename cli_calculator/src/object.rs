use std::cmp::Ordering;
use std::fmt;
use std::ops::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
    ArithmeticError
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result{
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "{x}"),
            Object::Bool(x) => {
                if *x {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::Nil => write!(f, "nil"),
            Object::ArithmeticError => panic!("Shouldn't be printing this???")
        }
    }
}

impl Sub for Object {
    type Output = Object;
    fn sub(self, other: Self) -> Object {
        match(self, other){
            (Object::Num(left), Object::Num(right)) => {
                Object::Num(left - right)
            }
            _ => Object::ArithmeticError
        }
    }
}

impl Div for Object {
    type Output = Object;
    fn div(self, other: Self) -> Object {
        match(self, other){
            (Object::Num(left), Object::Num(right)) => {
                if right == 0.0 {
                    Object::ArithmeticError
                } else {
                    Object::Num(left / right)
                }

            }
            _ => Object::ArithmeticError
        }
    }
}

impl Mul for Object {
    type Output = Object;
    fn mul(self, other: Self) -> Object {
        match(self, other){
            (Object::Num(left), Object::Num(right)) => {
                Object::Num(left * right)
            }
            _ => Object::ArithmeticError
        }
    }
}

impl Add for Object {
    type Output = Object;
    fn add(self, other: Self) -> Object {
        match(self, other){
            (Object::Num(left), Object::Num(right)) => {
                Object::Num(left + right)
            },
            (Object::Str(left), Object::Str(right)) => {
                Object::Str( format!("{}{}", left, right) )
            },
            (Object::Str(left), Object::Num(right)) => {
                Object::Str( format!("{}{}", left, right.to_string()) )
            },
            (Object::Num(left), Object::Str(right)) => {
                Object::Str( format!("{}{}", left.to_string(), right) )
            }
            _ => Object::ArithmeticError
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match(self, other){
            (Object::Nil, o) => {
                if o == &Object::Nil {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            (Object::Num(left), Object::Num(right)) => {
                return left.partial_cmp(right)
            },
            _ => None
        }
    }
}