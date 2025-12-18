use crate::{
    day04::part1::{parse_day_04, Field},
    util::point::Point,
};

pub fn solve_day_04_part_02(input: &str) -> u32 {
    let mut grid = parse_day_04(input);
    let mut result = 0;

    loop {
        let prev_result = result;
        let layer: Vec<(Point, Field)> = grid
            .iter()
            .filter_map(|(p, f)| {
                if f == &Field::PaperRoll
                    && p.neighbors_with_diagnonals()
                        .iter()
                        .filter(|&p2| grid.get(p2) == Some(&Field::PaperRoll))
                        .count()
                        < 4
                {
                    Some((*p, *f))
                } else {
                    None
                }
            })
            .collect();

        for (p, _) in layer {
            result += 1;
            grid.set(p, Field::Empty);
        }

        if result == prev_result {
            break;
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
