#![allow(unused_variables, dead_code)]
pub fn solve_day_07_part_02(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::util::file::read_string;
    use super::*;
    use indoc::indoc;

    #[test]
    fn should_solve_day_07_part_02() {
        let input = read_string("./src/day07/input.txt").unwrap();

        let solution = solve_day_07_part_02(&input);

	// assert_eq!(0, solution);
        println!("{solution}");
    }


    #[test]
    fn should_solve_day_07_part_02_sample() {
        let input = indoc! {"

        "}
        .trim();

        assert_eq!(0, solve_day_07_part_02(input));
    }
}
