pub fn solve_day_03_part_02(input: &str) -> u64 {
    input.lines().map(jolts).sum()
}

fn jolts(l: &str) -> u64 {
    let mut sum = 0;
    let mut offset = 0;

    let jolts: Vec<(usize, u32)> = l
        .chars()
        .map(|c| c.to_digit(10).expect("nan"))
        .enumerate()
        .collect();

    for i in (1..=12).rev() {
        let mut max_left: (usize, u32) = (0, 0);
        for jolt in jolts[offset..jolts.len() - (i - 1)].iter() {
            if jolt.1 > max_left.1 {
                max_left = *jolt;
            }
        }
        offset = max_left.0 + 1;
        sum += max_left.1 as u64 * 10u64.pow(i as u32 - 1);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_03_part_02() {
        let input = read_string("./src/day03/input.txt").unwrap();

        let solution = solve_day_03_part_02(&input);

        assert_eq!(171388730430281, solution);
    }

    #[test]
    fn should_solve_day_03_part_02_sample() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};

        assert_eq!(3121910778619, solve_day_03_part_02(input));
    }

    #[test]
    fn should_solve_day_03_part_02_sample01() {
        let input = indoc! {"
            987654321111111
        "};

        assert_eq!(987654321111, solve_day_03_part_02(input));
    }
}
