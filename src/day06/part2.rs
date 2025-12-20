use crate::{day06::part1::Op, util};

pub fn solve_day_06_part_02(input: &str) -> u128 {
    let rotated = util::string::rotate_clockwise_90(input);

    let mut result = 0;
    let mut group_aggr = 0;
    let mut op = Op::Add;
    rotated.lines().for_each(|mut line| {
        if line.starts_with("*") {
            op = Op::Mult;
            line = &line[1..];
        } else if line.starts_with("+") {
            op = Op::Add;
            line = &line[1..];
        }

        if !line.trim().is_empty() {
            let num = line
                .trim()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u128>()
                .expect("nan - col");
            group_aggr = match op {
                Op::Add => group_aggr + num,
                Op::Mult => {
                    if group_aggr != 0 {
                        group_aggr * num
                    } else {
                        num
                    }
                }
            }
        } else {
            result += group_aggr;
            group_aggr = 0;
        }
    });

    result += group_aggr;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_06_part_02() {
        let input = read_string("./src/day06/input.txt").unwrap();

        let solution = solve_day_06_part_02(&input);

        assert_eq!(7858808482092, solution)
    }

    #[test]
    fn should_solve_day_06_part_02_sample() {
        let input = indoc! {"
            123 328  51 64 
             45 64  387 23 
              6 98  215 314
            *   +   *   +  
        "};

        assert_eq!(3263827, solve_day_06_part_02(input));
    }
}
