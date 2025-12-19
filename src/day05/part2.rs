use crate::day05::part1::parse_day05;

pub fn solve_day_05_part_02(input: &str) -> u64 {
    let (ranges, _) = parse_day05(input);
    let mut result = 0;

    for (min, max) in ranges {
        result = result + max - (min - 1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_05_part_02() {
        let input = read_string("./src/day05/input.txt").unwrap();

        let solution = solve_day_05_part_02(&input);

        assert_eq!(353863745078671, solution);
    }

    #[test]
    fn should_solve_day_05_part_02_sample() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18
            
            1
            5
            8
            11
            17
            32
        "}
        .trim();

        assert_eq!(14, solve_day_05_part_02(input));
    }
}
