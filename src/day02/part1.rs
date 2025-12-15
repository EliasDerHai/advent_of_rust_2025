/**
* NOTE: get all invalid ids for a range A <-> B
* take A and split it (eg. 1234 -> 12; 12345 -> 12)
* dupe the split_A (eg. 12 -> 1212; 123 -> 123123)
* check if <= B -> if so increment spit_A and repeate the dupe-increment loop until > B
*/
pub fn solve_day_02_part_01(input: &str) -> u64 {
    let mut result = 0u64;
    for (lower, upper) in parse_day02(input) {
        let (mut half, mut full) = split_dupe(&lower);
        let lower = lower.parse().unwrap();

        while full <= upper {
            if full >= lower {
                result += full;
            }
            half += 1;
            full = format!("{half}{half}")
                .parse::<u64>()
                .expect("should be able to double halves");
        }
    }

    result
}

fn split_dupe(s: &str) -> (u64, u64) {
    let half = if s.len().is_multiple_of(2) {
        s[0..s.len() / 2].to_string()
    } else {
        10u64.pow(s.len() as u32 / 2).to_string()
    };

    let full = format!("{half}{half}")
        .parse::<u64>()
        .expect("should be able to double halves");
    (
        half.parse::<u64>()
            .expect("should be able to double halves"),
        full,
    )
}

fn parse_day02(input: &str) -> Vec<(String, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let (lower, upper) = range.split_once('-').expect("must contain -");
            (lower.into(), upper.parse::<u64>().expect("nan"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_02_part_01() {
        let input = read_string("./src/day02/input.txt").unwrap();

        let solution = solve_day_02_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_02_part_01_sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            .trim();

        assert_eq!(1227775554, solve_day_02_part_01(input));
    }

    #[test]
    fn debug_individual_ranges() {
        let test_cases = vec![
            ("61-71", vec![66]),
            ("1-16", vec![11]),
            ("17-30", vec![22]),
            ("34-46", vec![44]),
            ("48-59", vec![55]),
            ("72-87", vec![77]),
            ("92-115", vec![99]),
        ];

        for (range, expected) in test_cases {
            let result = solve_day_02_part_01(range);
            let expected_sum: u64 = expected.iter().sum();
            assert_eq!(expected_sum, result, "Failed for range: {}", range);
        }
    }
}
