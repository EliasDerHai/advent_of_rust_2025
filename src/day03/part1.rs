#![allow(unused_variables, dead_code)]
pub fn solve_day_03_part_01(input: &str) -> u32 {
    input.lines().map(jolts).sum()
}

pub fn jolts(l: &str) -> u32 {
    let mut max_left: (usize, u32) = (0, 0);
    let jolts: Vec<(usize, u32)> = l
        .chars()
        .map(|c| c.to_digit(10).expect("nan"))
        .enumerate()
        .collect();

    for jolt in jolts[..jolts.len() - 1].iter() {
        if jolt.1 > max_left.1 {
            max_left = *jolt;
        }
    }

    let mut max_right: (usize, u32) = (0, 0);
    for jolt in jolts[max_left.0 + 1..].iter() {
        if jolt.1 > max_right.1 {
            max_right = *jolt;
        }
    }

    max_right.1 + max_left.1 * 10
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_03_part_01() {
        let input = read_string("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_03_part_01_sample() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};

        assert_eq!(357, solve_day_03_part_01(input));
    }
}
