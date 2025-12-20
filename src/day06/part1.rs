use std::{iter::Rev, str::Lines};

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Mult,
}

pub fn solve_day_06_part_01(input: &str) -> u128 {
    let (lines, mut header) = parse_ops(input);

    for line in lines {
        for (num, (op, aggr)) in line.split_whitespace().zip(&mut header) {
            let num = num.parse::<u128>().expect("nan");
            match op {
                Op::Add => *aggr += num,
                Op::Mult => *aggr = if *aggr == 0 { num } else { *aggr * num },
            };
        }
    }

    header.into_iter().map(|(_, v)| v).sum()
}

fn parse_ops(input: &str) -> (Rev<Lines<'_>>, Vec<(Op, u128)>) {
    let mut lines = input.trim().lines().rev();
    let ops = lines.next().expect("non-empty");

    (
        lines,
        ops.split_whitespace()
            .map(|s| {
                let op = match s {
                    "*" => Op::Mult,
                    "+" => Op::Add,
                    o => panic!("unexpected op: {o}"),
                };
                (op, 0u128)
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_06_part_01() {
        let input = read_string("./src/day06/input.txt").unwrap();

        let solution = solve_day_06_part_01(&input);

        assert_eq!(4412382293768, solution);
    }

    #[test]
    fn should_solve_day_06_part_01_sample() {
        let input = indoc! {"
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
        "}
        .trim();

        assert_eq!(4277556, solve_day_06_part_01(input));
    }
}
