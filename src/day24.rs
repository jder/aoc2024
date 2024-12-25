use log::debug;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{collections::HashSet, fmt::Display, rc::Rc, str::FromStr};

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
    let (orig_values, mut expressions) = parse(input);

    // simplyfing assumption: swapping one gate is all that's needed to fix the next wrong z00 bit, starting with lsb
    // swapping means changing expresions[x] <-> expressions[y]

    let mut rng = ChaCha8Rng::seed_from_u64(1);

    loop {
        let first_wrong = find_next_wrong(&orig_values, &expressions, &mut rng);
        debug!("{}", first_wrong);

        let name = format!("z{:02}", first_wrong).leak();
        let (found, substitute) = recurse_substitute(
            &mut rng,
            &orig_values,
            &mut expressions,
            first_wrong,
            name,
            1_000,
        );

        println!("{found} -> {substitute}");

        let first_wrong_expr = expressions.remove(found).unwrap();
        let substitute_expr = expressions.remove(substitute).unwrap();

        expressions.insert(found, substitute_expr);
        expressions.insert(substitute, first_wrong_expr);
    }

    todo!()
}

fn recurse_substitute<'a>(
    rng: &mut ChaCha8Rng,
    orig_values: &HashMap<&'a str, bool>,
    expressions: &mut HashMap<&'a str, Expr<'a>>,
    target: usize,
    name: &'a str,
    count: usize,
) -> (&'a str, &'a str) {
    let mut candidates: HashSet<&'a str> = expressions
        .keys()
        .chain(orig_values.keys())
        .copied()
        .collect();

    let mut explored = Vec::new();

    for _ in 0..count {
        let x = rng.gen::<usize>() & ((1 << 45) - 1);
        let y = rng.gen::<usize>() & ((1 << 45) - 1);

        let mut values = orig_values.clone();
        let mut needed = HashMap::new();

        prep_values(x, y, &mut values);
        for output in variable_msb_first("z", &values, &expressions).iter().rev() {
            eval(output, &mut values, &expressions);
            debug!("{}: {:?}", output, &expressions[output]);
        }

        let z = to_usize("z", &values, &expressions);

        let expected_z = x + y;
        let incorrect_z = expected_z ^ z;

        if incorrect_z >> target & 1 == 1 {
            let expected = expected_z >> target & 1 == 1;

            populate_needed(name, expected, &mut needed, &expressions, &values);

            candidates.retain(|candidate| values[candidate] == expected);
            explored.push((x, y, z, expected_z, incorrect_z, values, needed));
        }

        // println!("{x}, {y}, {z}, {expected_z}, {incorrect_z}");
        // println!("x: {x:064b} x\ny: {y:064b} y\nz: {z:064b} z\ne: {expected_z:064b} e\nw: {incorrect_z:064b} w");
    }

    println!("CAndiadtes: {:?}", candidates);

    if candidates.len() == 1 {
        let substitute = candidates.iter().next().copied().unwrap();
        return (name, substitute);
    } else if candidates.len() > 1 {
        recurse_substitute(rng, orig_values, expressions, target, name, count * 2)
    } else {
        let flat_map = explored
            .iter()
            .flat_map(|(_, _, _, _, _, _, needed)| needed.keys())
            .counts();
        let needed_counts = flat_map
            .iter()
            .sorted_by_key(|(_, &count)| count)
            .rev()
            .collect::<Vec<_>>();

        for (name, _) in needed_counts.iter() {
            let needed_candidates = explored
                .iter()
                .map(|(_, _, _, _, _, _, needed)| needed.get(***name).clone())
                .fold(None, |acc: Option<HashSet<_>>, set| {
                    if let Some(set) = set {
                        if let Some(acc) = acc {
                            Some(acc.intersection(&set).copied().collect())
                        } else {
                            Some(set.clone())
                        }
                    } else {
                        acc
                    }
                });

            if let Some(candidates) = needed_candidates {
                if candidates.len() == 1 {
                    let substitute = candidates.iter().next().copied().unwrap();
                    return (name, substitute);
                } else {
                    return recurse_substitute(
                        rng,
                        orig_values,
                        expressions,
                        target,
                        name,
                        count * 2,
                    );
                }
            }
        }
    }
}

fn populate_needed<'a>(
    name: &'a str,
    expected: bool,
    needed: &mut HashMap<&'a str, HashSet<&'a str>>,
    expressions: &HashMap<&'a str, Expr<'a>>,
    values: &HashMap<&'a str, bool>,
) {
    if values[name] == expected {
        return;
    }

    if name.starts_with("x") || name.starts_with("y") {
        return;
    }

    needed.insert(
        name,
        values
            .iter()
            .filter(|(k, v)| **v == expected)
            .map(|(k, _)| *k)
            .collect(),
    );

    let expr = &expressions[name];
    match expr.op {
        Op::And => {
            if expected {
                if values[expr.a] {
                    assert!(expected == expr.op.eval(values[expr.a], true));
                    populate_needed(expr.b, true, needed, expressions, values);
                }

                if values[expr.b] {
                    assert!(expected == expr.op.eval(true, values[expr.b]));
                    populate_needed(expr.a, true, needed, expressions, values);
                }
            } else {
                assert!(expected == expr.op.eval(values[expr.a], false));
                populate_needed(expr.b, false, needed, expressions, values);
                assert!(expected == expr.op.eval(false, values[expr.b]));
                populate_needed(expr.a, false, needed, expressions, values);
            }
        }
        Op::Or => {
            if expected {
                assert!(expected == expr.op.eval(values[expr.a], true));
                populate_needed(expr.b, true, needed, expressions, values);
                assert!(expected == expr.op.eval(true, values[expr.b]));
                populate_needed(expr.a, true, needed, expressions, values);
            } else {
                if !values[expr.a] {
                    assert!(expected == expr.op.eval(values[expr.a], false));
                    populate_needed(expr.b, false, needed, expressions, values);
                }

                if !values[expr.b] {
                    assert!(expected == expr.op.eval(false, values[expr.b]));
                    populate_needed(expr.a, false, needed, expressions, values);
                }
            }
        }
        Op::Xor => {
            if expected {
                if values[expr.a] {
                    assert!(expected == expr.op.eval(values[expr.a], false));
                    populate_needed(expr.b, false, needed, expressions, values);
                } else {
                    assert!(expected == expr.op.eval(values[expr.a], true));
                    populate_needed(expr.b, true, needed, expressions, values);
                }

                if values[expr.b] {
                    assert!(expected == expr.op.eval(false, values[expr.b]));
                    populate_needed(expr.a, false, needed, expressions, values);
                } else {
                    assert!(expected == expr.op.eval(true, values[expr.b]));
                    populate_needed(expr.a, true, needed, expressions, values);
                }
            } else {
                if values[expr.a] {
                    assert!(expected == expr.op.eval(values[expr.a], true));
                    populate_needed(expr.b, true, needed, expressions, values);
                } else {
                    assert!(expected == expr.op.eval(values[expr.a], false));
                    populate_needed(expr.b, false, needed, expressions, values);
                }

                if values[expr.b] {
                    assert!(expected == expr.op.eval(true, values[expr.b]));
                    populate_needed(expr.a, true, needed, expressions, values);
                } else {
                    assert!(expected == expr.op.eval(false, values[expr.b]));
                    populate_needed(expr.a, false, needed, expressions, values);
                }
            }
        }
    }
}

fn find_next_wrong(
    orig_values: &HashMap<&str, bool>,
    expressions: &HashMap<&str, Expr<'_>>,
    rng: &mut ChaCha8Rng,
) -> usize {
    let mut all_incorrect = 0;

    for _ in 0..1000 {
        let x = rng.gen::<usize>() & ((1 << 45) - 1);
        let y = rng.gen::<usize>() & ((1 << 45) - 1);

        let mut values = orig_values.clone();

        prep_values(x, y, &mut values);
        for output in variable_msb_first("z", &values, expressions).iter().rev() {
            eval(output, &mut values, expressions);
            debug!("{}: {:?}", output, &expressions[output]);
        }

        let z = to_usize("z", &values, expressions);

        let expected_z = x + y;
        let incorrect_z = expected_z ^ z;

        // println!("{x}, {y}, {z}, {expected_z}, {incorrect_z}");
        // println!("x: {x:064b} x\ny: {y:064b} y\nz: {z:064b} z\ne: {expected_z:064b} e\nw: {incorrect_z:064b} w");

        all_incorrect |= incorrect_z;
    }

    println!("{:064b}", all_incorrect);

    let first_wrong = all_incorrect.trailing_zeros() as usize;
    first_wrong
}

fn find_substitute<'a>(
    rng: &mut ChaCha8Rng,
    orig_values: &HashMap<&'a str, bool>,
    expressions: &HashMap<&'a str, Expr<'a>>,
    target: usize,
) -> Option<&'a str> {
    let mut candidates: HashSet<&'a str> = expressions
        .keys()
        .chain(orig_values.keys())
        .copied()
        .collect();

    for _ in 0..10000 {
        let x = rng.gen::<usize>() & ((1 << 45) - 1);
        let y = rng.gen::<usize>() & ((1 << 45) - 1);

        let mut values = orig_values.clone();

        prep_values(x, y, &mut values);
        for output in variable_msb_first("z", &values, &expressions).iter().rev() {
            eval(output, &mut values, &expressions);
            debug!("{}: {:?}", output, &expressions[output]);
        }

        let z = to_usize("z", &values, &expressions);

        let expected_z = x + y;
        let incorrect_z = expected_z ^ z;

        if incorrect_z >> target & 1 == 1 {
            let expected = expected_z >> target & 1 == 1;

            candidates.retain(|candidate| values[candidate] == expected);
        }

        // println!("{x}, {y}, {z}, {expected_z}, {incorrect_z}");
        // println!("x: {x:064b} x\ny: {y:064b} y\nz: {z:064b} z\ne: {expected_z:064b} e\nw: {incorrect_z:064b} w");
    }

    println!("CAndiadtes: {:?}", candidates);

    candidates.iter().next().copied()
}

fn prep_values<'a>(x: usize, y: usize, values: &mut HashMap<&'a str, bool>) {
    for (k, v) in values {
        if k.starts_with("x") {
            *v = (x >> k[1..].parse::<usize>().unwrap()) & 1 == 1;
        } else if k.starts_with("y") {
            *v = (y >> k[1..].parse::<usize>().unwrap()) & 1 == 1;
        }
    }
}

enum Evaluation {
    Correct,   // never seen wrong
    Incorrect, // seen wrong when no way one child is wrong
}
