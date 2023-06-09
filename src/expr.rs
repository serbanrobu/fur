use std::{fmt, ops::Add};

use color_eyre::{
    eyre::{eyre, ContextCompat},
    Result,
};

use crate::{Context, Env, Int, Level, Name, Prec, Type, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Ann(Box<Expr>, Box<Expr>),
    U(Level),
    Int,
    IntLit(Int),
    Add(Box<Expr>, Box<Expr>),
    Var(Name),
    Trivial,
    Sole,
}

impl Expr {
    pub fn alpha_eq(&self, other: &Self) -> bool {
        self == other
    }

    // pub fn vars(&self) -> Vars {
    //     Vars { stack: vec![self] }
    // }

    pub fn check(&self, t: &Type, ctx: &Context) -> Result<()> {
        match (self, t) {
            (Self::U(i), Type::U(j)) if i < j => Ok(()),
            (Self::Int, Type::U(_)) => Ok(()),
            (Self::IntLit(_), Type::Int) => Ok(()),
            (Self::Trivial, Type::U(_)) => Ok(()),
            (Self::Sole, Type::Trivial) => Ok(()),
            _ => {
                let t_ = self.infer(ctx)?;

                if !t.quote().alpha_eq(&t_.quote()) {
                    return Err(eyre!("Type mismatch"));
                }

                Ok(())
            }
        }
    }

    pub fn infer(&self, ctx: &Context) -> Result<Type> {
        match self {
            Self::Ann(e_1, e_2) => {
                e_2.check(&Type::U(Level::MAX), ctx)?;

                let env: Env = ctx
                    .iter()
                    .filter_map(|(k, (_, v))| v.as_ref().map(|v| (k.to_owned(), v.to_owned())))
                    .collect();

                let t = e_2.eval(&env);

                e_1.check(&t, ctx)?;

                Ok(t)
            }
            &Self::U(i) if i < Level::MAX => Ok(Type::U(i + 1)),
            Self::Int => Ok(Type::U(0)),
            Self::IntLit(_) => Ok(Type::Int),
            Self::Add(e_1, e_2) => {
                let t = Type::Int;
                e_1.check(&t, ctx)?;
                e_2.check(&t, ctx)?;
                Ok(t)
            }
            Self::Var(x) => ctx.get(x).map(|(t, _)| t).cloned().wrap_err("Not found"),
            Self::Trivial => Ok(Type::U(0)),
            Self::Sole => Ok(Type::Trivial),
            _ => Err(eyre!("Failed to synthesize a type")),
        }
    }

    pub fn eval(&self, env: &Env) -> Value {
        match self {
            Self::Ann(e, _) => e.eval(env),
            &Self::U(i) => Value::U(i),
            Self::Int => Value::Int,
            &Self::IntLit(i) => i.into(),
            Self::Add(e_1, e_2) => e_1.eval(env) + e_2.eval(env),
            Self::Var(x) => env
                .get(x)
                .cloned()
                .unwrap_or_else(|| Value::var(x.to_owned())),
            Self::Trivial => Value::Trivial,
            Self::Sole => Value::Sole,
        }
    }

    fn fmt_parens(&self, f: &mut fmt::Formatter<'_>, condition: bool) -> fmt::Result {
        if condition {
            write!(f, "({self})")
        } else {
            write!(f, "{self}")
        }
    }

    fn prec(&self) -> Prec {
        match self {
            Self::Ann(_, _) => Prec::Ann,
            Self::Add(_, _) => Prec::Add,
            _ => Prec::Atom,
        }
    }
}

impl From<Int> for Expr {
    fn from(value: Int) -> Self {
        Self::IntLit(value)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ann(e_1, e_2) => write!(f, "{e_1} : {e_2}"),
            Self::U(i) => write!(f, "U({i})"),
            Self::Int => "Int".fmt(f),
            Self::IntLit(i) => i.fmt(f),
            Self::Add(e_1, e_2) => {
                e_1.fmt_parens(f, self.prec() > e_1.prec())?;
                write!(f, " + ")?;
                e_2.fmt_parens(f, self.prec() > e_2.prec())
            }
            Self::Var(x) => x.fmt(f),
            Self::Trivial => "Trivial".fmt(f),
            Self::Sole => "sole".fmt(f),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Add(self.into(), rhs.into())
    }
}

// pub struct Vars<'a> {
//     stack: Vec<&'a Expr>,
// }
//
// impl<'a> Iterator for Vars<'a> {
//     type Item = &'a str;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let expr = self.stack.pop()?;
//
//         match expr {
//             Expr::U(_) | Expr::Int | Expr::IntLit(_) | Expr::Trivial | Expr::Sole => self.next(),
//             Expr::Add(e_1, e_2) => {
//                 self.stack.push(e_2);
//                 self.stack.push(e_1);
//                 self.next()
//             }
//             Expr::Var(x) => Some(x),
//             Expr::Io(e) => {
//                 self.stack.push(e);
//                 self.next()
//             }
//             Expr::IoLet(x, e) => {
//                 self.stack.push(e);
//                 self.next()
//             }
//             Expr::Print(e) => {
//                 self.stack.push(e);
//                 self.next()
//             }
//         }
//     }
// }
