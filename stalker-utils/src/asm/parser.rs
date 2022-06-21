use crate::asm::inst::*;
use crate::error::Error;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, hex_digit1},
    combinator::{eof, map, map_res, opt, peek},
    multi::{many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn op(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

fn hex_i64(input: &str) -> IResult<&str, i64> {
    map_res(preceded(tag("0x"), hex_digit1), |out: &str| {
        i128::from_str_radix(out, 16).map(|a| a as i64)
    })(input)
}

fn digit_i64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |out: &str| out.parse::<i64>())(input)
}

fn num_i64(input: &str) -> IResult<&str, i64> {
    alt((hex_i64, digit_i64))(input)
}

fn arg_imm(input: &str) -> IResult<&str, Arg> {
    map(num_i64, Arg::Imm)(input)
}

fn arg_sym(input: &str) -> IResult<&str, Arg> {
    map(
        terminated(take_while1(|_| true), peek(alt((tag(","), tag(";"), eof)))),
        |s: &str| Arg::Sym(s.to_string()),
    )(input)
}

fn arg_reg(input: &str) -> IResult<&str, Arg> {
    map(
        terminated(alphanumeric1, peek(alt((tag(","), tag(";"), eof)))),
        |s: &str| Arg::Reg(s.to_string()),
    )(input)
}

fn arg_shft(input: &str) -> IResult<&str, RegShft> {
    alt((
        map(
            tuple((
                alpha1,
                char(' '),
                alt((char('-'), char('+'))),
                char(' '),
                num_i64,
            )),
            |(reg, _, sign, _, n)| RegShft {
                reg: Some(reg.to_string()),
                shft: match sign {
                    '-' => Some(-n),
                    _ => Some(n),
                },
            },
        ),
        map(num_i64, |n: i64| RegShft {
            reg: None,
            shft: Some(n),
        }),
        map(alpha1, |s: &str| RegShft {
            reg: Some(s.to_string()),
            shft: None,
        }),
    ))(input)
}

fn arg_mem(input: &str) -> IResult<&str, Arg> {
    map(
        tuple((
            opt(terminated(alpha1, char(' '))),
            opt(terminated(alpha1, char(':'))),
            delimited(char('['), arg_shft, char(']')),
        )),
        |(l, s, v): (Option<&str>, Option<&str>, RegShft)| {
            Arg::Mem(Addr {
                value: v,
                sel: s.map(|v| v.to_string()),
                len: l.map(|v| v.to_string()),
            })
        },
    )(input)
}

pub fn arg_exp(input: &str) -> IResult<&str, Arg> {
    map(
        separated_pair(alpha1, char(' '), take_while1(|_| true)),
        |(op, arg): (&str, &str)| Arg::Expr(op.to_string(), arg.to_string()),
    )(input)
}

fn arg(input: &str) -> IResult<&str, Arg> {
    alt((arg_imm, arg_mem, arg_exp, arg_reg, arg_sym))(input)
}

fn inst(input: &str) -> IResult<&str, Inst> {
    let (input, (op, args, _)) = tuple((
        op,
        opt(preceded(char(' '), separated_list0(tag(", "), arg))),
        alt((eof, tag(";"))),
    ))(input)?;
    Ok((
        input,
        Inst {
            op: op.to_string(),
            args,
        },
    ))
}

fn insts(input: &str) -> IResult<&str, Vec<Inst>> {
    many1(preceded(opt(tag(" ")), inst))(input)
}

pub struct AsmParser;

impl AsmParser {
    pub fn parse(input: &str) -> Result<Inst, Error> {
        match inst(input) {
            Ok((_, inst)) => Ok(inst),
            Err(e) => Err(Error::Parse(format!("{}", e))),
        }
    }

    pub fn parse_many(input: &str) -> Result<Vec<Inst>, Error> {
        match insts(input) {
            Ok((_, insts)) => Ok(insts),
            Err(e) => Err(Error::Parse(format!("{}", e))),
        }
    }
}
