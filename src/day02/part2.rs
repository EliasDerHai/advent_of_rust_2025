use std::collections::HashSet;

pub fn solve_day_02_part_02(input: &str) -> u64 {
    let mut cache: HashSet<u64> = HashSet::new();
    for (lower, upper) in parse_day02(input) {
        find_repeating_numbers(lower, upper, &mut cache);
    }
    cache.iter().sum()
}

fn find_repeating_numbers(lower: u64, upper: u64, cache: &mut HashSet<u64>) {
    let min_digits = lower.to_string().len();
    let max_digits = upper.to_string().len();

    for total_digits in min_digits..=max_digits {
        for group_size in 1..=total_digits / 2 {
            if total_digits % group_size != 0 {
                continue;
            }

            let repetitions = total_digits / group_size;

            let pattern_min = 10u64.pow(group_size as u32 - 1);
            let pattern_max = 10u64.pow(group_size as u32) - 1;

            for pattern in pattern_min..=pattern_max {
                let repeated_num = repeat_pattern(pattern, repetitions);

                if repeated_num >= lower && repeated_num <= upper {
                    cache.insert(repeated_num);
                }
            }
        }
    }
}

fn repeat_pattern(pattern: u64, times: usize) -> u64 {
    let pattern_str = pattern.to_string();
    let repeated_str = pattern_str.repeat(times);
    repeated_str.parse::<u64>().unwrap_or(u64::MAX)
}

fn parse_day02(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (lower, upper) = range.split_once('-').expect("must contain -");
            (
                lower.parse::<u64>().expect("nan"),
                upper.parse::<u64>().expect("nan"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_02_part_02() {
        let input = read_string("./src/day02/input.txt").unwrap();

        let solution = solve_day_02_part_02(&input);

        assert_eq!(85513235135, solution);
    }

    #[test]
    fn should_solve_day_02_part_02_sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .trim();

        assert_eq!(4174379265, solve_day_02_part_02(input));
    }
}
