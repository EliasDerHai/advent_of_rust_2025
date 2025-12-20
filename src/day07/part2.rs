use std::collections::HashMap;

pub fn solve_day_07_part_02(input: &str) -> u64 {
    let mut rays: HashMap<i32, u64> = HashMap::new();
    let mut it = input.lines();

    let start = it
        .next()
        .expect("empty")
        .chars()
        .position(|c| c == 'S')
        .expect("no S found");

    rays.insert(start as i32, 1);

    for line in it {
        for (pos, c) in line.chars().enumerate() {
            let pos = pos as i32;
            if c == '^' {
                if let Some(p_v) = rays.remove(&pos) {
                    let l = rays.get(&(pos - 1)).cloned().unwrap_or(0) + p_v;
                    rays.insert(pos - 1, l);
                    let r = rays.get(&(pos + 1)).cloned().unwrap_or(0) + p_v;
                    rays.insert(pos + 1, r);
                }
            }
        }
    }

    rays.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_07_part_02() {
        let input = read_string("./src/day07/input.txt").unwrap();

        let solution = solve_day_07_part_02(&input);

        assert_eq!(231507396180012, solution);
    }

    #[test]
    fn should_solve_day_07_part_02_sample() {
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

        assert_eq!(40, solve_day_07_part_02(input));
    }
}
