pub fn solve_day_01_part_01(input: &str) -> i32 {
    let mut v = 50;
    let mut c = 0;
    for el in parse_day_01(input) {
        v = (v + el) % 100;
        if v == 0 {
            c += 1;
        }
    }
    c
}

pub fn parse_day_01(input: &str) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(|line| {
            match line.trim().to_lowercase() {
                x if x.starts_with("r") => x[1..].parse::<i32>(),
                x if x.starts_with("l") => x[1..].parse::<i32>().map(|v| -v),
                o => panic!("unexpected ${o}"),
            }
            .expect("unparseable")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_01_part_01() {
        let input = read_string("./src/day01/input.txt").unwrap();

        let solution = solve_day_01_part_01(&input);

        println!("{solution}");
    }

    #[test]
    fn should_solve_day_01_part_01_sample() {
        let input = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "
        .trim();

        assert_eq!(3, solve_day_01_part_01(input));
    }
}
