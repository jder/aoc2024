use std::{collections::HashSet, fmt::Display, rc::Rc, str::FromStr};

use log::debug;

use crate::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err(()),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And => write!(f, "&"),
            Op::Or => write!(f, "|"),
            Op::Xor => write!(f, "^"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Expr<'a> {
    op: Op,
    a: &'a str,
    b: &'a str,
}

fn eval<'a>(
    var: &'a str,
    values: &mut HashMap<&'a str, bool>,
    expressions: &HashMap<&'a str, Expr<'a>>,
) -> bool {
    if let Some(value) = values.get(var) {
        return *value;
    }

    let expr = &expressions[var];
    let a = eval(expr.a, values, expressions);
    let b = eval(expr.b, values, expressions);

    let value = expr.op.eval(a, b);
    values.insert(var, value);

    value
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let (mut values, expressions) = parse(input);

    for var in variable_msb_first("z", &values, &expressions) {
        eval(var, &mut values, &expressions);
    }

    let result = to_usize("z", &values, &expressions);

    result
}

fn to_usize(
    prefix: &str,
    values: &HashMap<&str, bool>,
    expressions: &HashMap<&str, Expr<'_>>,
) -> usize {
    variable_msb_first(prefix, values, expressions)
        .into_iter()
        .fold(0, |total, key| total << 1 | (values[key] as usize))
}

fn variable_msb_first<'a, 'b>(
    prefix: &str,
    values: &'b HashMap<&'a str, bool>,
    expressions: &'b HashMap<&'a str, Expr<'a>>,
) -> Vec<&'a str> {
    expressions
        .keys()
        .chain(values.keys())
        .filter(|key| key.starts_with(prefix))
        .sorted()
        .unique()
        .rev()
        .copied()
        .collect()
}

fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Expr<'_>>) {
    let (values_lines, gate_lines) = input.split("\n\n").collect_tuple().unwrap();

    let values_regex = Regex::new(r"([^:]+): (\d+)").unwrap();
    let values: HashMap<&str, bool> = values_lines
        .lines()
        .map(|line| {
            let (_, [name, value]) = values_regex.captures(line).unwrap().extract();
            (name, value.parse::<u8>().unwrap() != 0)
        })
        .collect();

    let gates_regex =
        Regex::new(r"([[:alnum:]]+) ([A-Z]+) ([[:alnum:]]+) -> ([[:alnum:]]+)").unwrap();
    let gates: HashMap<&str, Expr> = gate_lines
        .lines()
        .map(|line| {
            let (_, [a, op, b, out]) = gates_regex.captures(line).unwrap().extract();
            (
                out,
                Expr {
                    op: op.parse().unwrap(),
                    a,
                    b,
                },
            )
        })
        .collect();

    (values, gates)
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let (mut values, expressions) = parse(input);

    for output in variable_msb_first("z", &values, &expressions).iter().rev() {
        eval(output, &mut values, &expressions);
        debug!("{}: {:?}", output, &expressions[output]);
    }

    let x = to_usize("x", &values, &expressions);
    let y = to_usize("y", &values, &expressions);
    let z = to_usize("z", &values, &expressions);

    let expected_z = x + y;
    let incorrect_z = expected_z ^ z;

    println!("{x}, {y}, {z}, {expected_z}, {incorrect_z}");
    println!("x: {x:064b} x\ny: {y:064b} y\nz: {z:064b} z\ne: {expected_z:064b} e\nw: {incorrect_z:064b} w");

    // simplyfing assumption: swapping one gate is all that's needed to fix the next wrong z00 bit, starting with lsb
    // swapping means changing expresions[x] <-> expressions[y]

    todo!()
}

enum Evaluation {
    Correct,   // never seen wrong
    Incorrect, // seen wrong when no way one child is wrong
}
