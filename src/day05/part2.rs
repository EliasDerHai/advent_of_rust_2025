#![allow(unused_variables, dead_code)]
pub fn solve_day_05_part_02(input: &str) -> u32 {
    todo!()
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

        println!("{solution}");
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

        assert_eq!(3, solve_day_05_part_02(input));
    }
}
