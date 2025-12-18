#![allow(unused_variables, dead_code)]

#[derive(Debug, PartialEq, Eq)]
enum Field {
    PaperRoll,
    Empty,
}

use crate::util::grid::Grid;
pub fn solve_day_04_part_01(input: &str) -> u32 {
    let grid = Grid::from(input).map(|c| {
        if c == '@' {
            Field::PaperRoll
        } else {
            Field::Empty
        }
    });

    grid.iter()
        .filter(|&(p, f)| {
            f == &Field::PaperRoll
                && p.neighbors_with_diagnonals()
                    .iter()
                    .filter(|&p2| grid.get(p2) == Some(&Field::PaperRoll))
                    .count()
                    < 4
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_04_part_01() {
        let input = read_string("./src/day04/input.txt").unwrap();

        let solution = solve_day_04_part_01(&input);

        assert_eq!(1363, solution);
    }

    #[test]
    fn should_solve_day_04_part_01_sample() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "}
        .trim();

        assert_eq!(13, solve_day_04_part_01(input));
    }
}
