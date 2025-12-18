use crate::day04::part1::{parse_day_04, Field};

use std::collections::VecDeque;

pub fn solve_day_04_part_02(input: &str) -> u32 {
    let mut grid = parse_day_04(input);
    let mut result = 0;
    let mut queue = VecDeque::new();

    for (p, f) in grid.iter() {
        if f == &Field::PaperRoll {
            let neighbor_count = p
                .neighbors_with_diagnonals()
                .iter()
                .filter(|&p2| grid.get(p2) == Some(&Field::PaperRoll))
                .count();
            if neighbor_count < 4 {
                queue.push_back(*p);
            }
        }
    }

    while let Some(p) = queue.pop_front() {
        if grid.get(&p) != Some(&Field::PaperRoll) {
            continue;
        }

        let neighbor_count = p
            .neighbors_with_diagnonals()
            .iter()
            .filter(|&p2| grid.get(p2) == Some(&Field::PaperRoll))
            .count();

        if neighbor_count < 4 {
            grid.remove(&p);
            result += 1;

            for neighbor in p.neighbors_with_diagnonals() {
                if grid.get(&neighbor) == Some(&Field::PaperRoll) {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_04_part_02() {
        let input = read_string("./src/day04/input.txt").unwrap();

        let solution = solve_day_04_part_02(&input);

        assert_eq!(8184, solution)
    }

    #[test]
    fn should_solve_day_04_part_02_sample() {
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

        assert_eq!(43, solve_day_04_part_02(input));
    }
}
