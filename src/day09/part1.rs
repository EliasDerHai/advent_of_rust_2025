pub fn solve_day_09_part_01(input: &str) -> u64 {
    let parsed = parse(input);
    let mut result = 0;
    for &(x1, y1) in &parsed {
        for &(x2, y2) in &parsed {
            let delt_x = (x1.abs_diff(x2) + 1) as u64;
            let delt_y = (y1.abs_diff(y2) + 1) as u64;
            let area = delt_x * delt_y;

            result = result.max(area);
        }
    }
    result
}

pub fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').expect("line must contain ','");
            (x.parse().expect("nan"), y.parse().expect("nan"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_09_part_01() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_01(&input);

        assert_eq!(4733727792, solution);
    }

    #[test]
    fn should_solve_day_09_part_01_sample() {
        let input = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "}
        .trim();

        assert_eq!(50, solve_day_09_part_01(input));
    }
}
