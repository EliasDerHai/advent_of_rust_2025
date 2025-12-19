use itertools::Itertools;

pub fn solve_day_05_part_01(input: &str) -> u64 {
    let (ranges, ingredients) = parse(input);
    let mut result = 0;

    for ing in ingredients {
        if ranges.iter().any(|&(min, max)| min <= ing && max >= ing) {
            result += 1;
        }
    }

    result
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").expect("formatting");

    let ranges = ranges
        .lines()
        .map(|l| {
            let (min, max) = l.trim().split_once("-").expect("expected -");

            (
                min.parse::<u64>().expect("nan"),
                max.parse::<u64>().expect("nan"),
            )
        })
        .sorted_unstable_by_key(|(min, _)| *min);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (min, max) in ranges {
        if let Some(last) = merged.last_mut() {
            if min <= last.1 + 1 {
                last.1 = last.1.max(max);
                continue;
            }
        }
        merged.push((min, max));
    }

    let ingredients = ingredients
        .lines()
        .map(|l| l.trim().parse::<u64>().expect("nan"))
        .collect();

    (merged, ingredients)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_05_part_01() {
        let input = read_string("./src/day05/input.txt").unwrap();

        let solution = solve_day_05_part_01(&input);

        assert_eq!(782, solution);
    }

    #[test]
    fn should_solve_day_05_part_01_sample() {
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

        assert_eq!(3, solve_day_05_part_01(input));
    }
}
