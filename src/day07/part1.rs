use std::collections::HashSet;

pub fn solve_day_07_part_01(input: &str) -> u32 {
    let mut rays: HashSet<i32> = HashSet::new();
    let mut result = 0;
    let mut it = input.lines();

    let start = it
        .next()
        .expect("empty")
        .chars()
        .position(|c| c == 'S')
        .expect("no S found");

    rays.insert(start as i32);

    for line in it {
        for (pos, c) in line.chars().enumerate() {
            let pos = pos as i32;
            if c == '^' && rays.contains(&pos) {
                rays.remove(&pos);
                rays.insert(pos - 1);
                rays.insert(pos + 1);
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_07_part_01() {
        let input = read_string("./src/day07/input.txt").unwrap();

        let solution = solve_day_07_part_01(&input);

        assert_eq!(1717, solution);
    }

    #[test]
    fn should_solve_day_07_part_01_sample() {
        let input = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "}
        .trim();

        assert_eq!(21, solve_day_07_part_01(input));
    }
}
