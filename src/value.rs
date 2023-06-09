use std::ops::Add;

use crate::{Expr, Int, Level, Name};

#[derive(Clone, Debug)]
pub enum Neutral {
    Add(Box<Neutral>, Box<Value>),
    Var(Name),
}

impl Neutral {
    fn quote(&self) -> Expr {
        match self {
            Self::Add(v, n) => v.quote() + n.quote(),
            Self::Var(x) => Expr::Var(x.to_owned()),
        }
    }
}

impl Add<Value> for Neutral {
    type Output = Self;

    fn add(self, rhs: Value) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    U(Level),
    Int,
    IntLit(Int),
    Neutral(Neutral),
    Trivial,
    Sole,
}

impl Value {
    pub fn quote(&self) -> Expr {
        match self {
            &Self::U(i) => Expr::U(i),
            Self::Int => Expr::Int,
            &Self::IntLit(i) => i.into(),
            Self::Neutral(n) => n.quote(),
            Self::Trivial => Expr::Trivial,
            Self::Sole => Expr::Sole,
        }
    }

    pub fn var(x: Name) -> Self {
        Neutral::Var(x).into()
    }
}

impl From<Int> for Value {
    fn from(value: Int) -> Self {
        Self::IntLit(value)
    }
}

impl From<Neutral> for Value {
    fn from(value: Neutral) -> Self {
        Self::Neutral(value)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::IntLit(i), Self::IntLit(j)) => (i + j).into(),
            (Self::Neutral(n), v) | (v, Self::Neutral(n)) => (n + v).into(),
            _ => unreachable!(),
        }
    }
}
