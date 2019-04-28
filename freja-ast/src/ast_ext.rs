use super::ast::{Literal, Number};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Double(n) => write!(f, "{}", n),
            Number::Integer(n) => write!(f, "{}", n),
        }
    }
}

macro_rules! impl_op {
    ($trait: ident, $name: ident, $op: tt) => {
        impl $trait for Number {
            type Output = Number;
            fn $name(self, rhs: Number) -> Self::Output {
                match self {
                    Number::Double(f) => match rhs {
                        Number::Double(ff) => Number::Double(f $op ff),
                        Number::Integer(i) => Number::Double(f $op (i as f64)),
                    },
                    Number::Integer(i) => match rhs {
                        Number::Double(f) => Number::Double((i as f64) $op f),
                        Number::Integer(ii) => Number::Integer(i $op ii),
                    },
                }
            }
        }

        impl $trait for &Number {
            type Output = Number;
            fn $name(self, rhs: &Number) -> Self::Output {
                match self {
                    Number::Double(f) => match rhs {
                        Number::Double(ff) => Number::Double(f $op ff),
                        Number::Integer(i) => Number::Double(f $op (*i as f64)),
                    },
                    Number::Integer(i) => match rhs {
                        Number::Double(f) => Number::Double((*i as f64) $op f),
                        Number::Integer(ii) => Number::Integer(i $op ii),
                    },
                }
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Literal::String(n) => write!(f, "{}", n),
            Literal::Number(n) => <Number as fmt::Display>::fmt(n, f),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}
