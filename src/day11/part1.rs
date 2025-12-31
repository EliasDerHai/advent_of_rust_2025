use std::collections::{HashMap, VecDeque};

pub fn solve_day_11_part_01(input: &str) -> u32 {
    let map = parse(input);
    let mut backlog = VecDeque::new();
    backlog.push_back(map.get("you").unwrap());
    let mut count = 0;

    while let Some(next) = backlog.pop_front() {
        for n in next {
            if n == "out" {
                count += 1;
            } else {
                backlog.push_back(map.get(n).unwrap());
            }
        }
    }

    count
}

pub fn parse(input: &str) -> HashMap<String, Vec<String>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (key, values) = line.trim().split_once(": ").unwrap();

            (
                key.to_string(),
                values
                    .split(" ")
                    .filter(|elem| !elem.trim().is_empty())
                    .map(|elem| elem.to_string())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_11_part_01() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11_part_01(&input);

        assert_eq!(690, solution);
    }

    #[test]
    fn should_solve_day_11_part_01_sample() {
        let input = indoc! {"
            aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out
        "}
        .trim();

        assert_eq!(5, solve_day_11_part_01(input));
    }
}
