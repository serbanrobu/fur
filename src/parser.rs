use std::{ops::Add, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, i64, multispace0, u8},
    combinator::{eof, map, opt, recognize, value},
    error::Error,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    Finish, IResult,
};

use crate::{Expr, Name};

fn parse_name(input: &str) -> IResult<&str, Name> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag("\'")))),
        )),
        |s: &str| s.to_string(),
    )(input)
}

fn parse_add(input: &str) -> IResult<&str, Expr> {
    let parser = alt((
        parens(ws(parse_expr)),
        value(Expr::Int, tag("Int")),
        value(Expr::Trivial, tag("Trivial")),
        value(Expr::Sole, tag("sole")),
        map(i64, Expr::from),
        map(
            preceded(pair(char('U'), multispace0), parens(ws(u8))),
            Expr::U,
        ),
        map(parse_name, Expr::Var),
    ));

    map(separated_list1(ws(char('+')), parser), |xs| {
        xs.into_iter().reduce(Add::add).unwrap()
    })(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    map(
        pair(parse_add, opt(preceded(ws(char(':')), parse_add))),
        |(e_1, e_2)| match e_2 {
            Some(e_2) => Expr::Ann(e_1.into(), e_2.into()),
            None => e_1,
        },
    )(input)
}

impl FromStr for Expr {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_remaining, expr) = terminated(parse_expr, eof)(s.trim())
            .finish()
            .map_err(|e| Error {
                input: e.input.to_string(),
                code: e.code,
            })?;

        Ok(expr)
    }
}

fn parens<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(char('('), inner, char(')'))
}

fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}
