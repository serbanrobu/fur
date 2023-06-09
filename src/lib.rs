use std::collections::HashMap;

pub use expr::Expr;
pub use value::Value;

mod expr;
pub mod parser;
mod value;

pub type Int = i64;

pub type Name = String;

pub type Level = u8;

pub type Type = Value;

pub type Context = HashMap<Name, (Type, Option<Value>)>;

pub type Env = HashMap<Name, Value>;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Prec {
    Ann,
    Add,
    Atom,
}
