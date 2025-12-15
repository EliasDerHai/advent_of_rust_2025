pub fn solve_day_01_part_02(input: &str) -> u32 {
    let mut v = 50;
    let mut c = 0u32;

    for delta in super::part1::parse_day_01(input) {
        let mul = delta / 100;
        let v_orig = v;

        c += mul.unsigned_abs();
        v += delta % 100;

        if v >= 100 {
            v -= 100;
            c += 1;
        } else if v <= 0 {
            if v_orig != 0 {
                c += 1;
            }
            if v < 0 {
                v += 100;
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_01_part_02() {
        let input = read_string("./src/day01/input.txt").unwrap();

        let solution = solve_day_01_part_02(&input);

        assert_eq!(6858, solution)
    }

    #[test]
    fn should_solve_day_01_part_02_sample() {
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

        assert_eq!(6, solve_day_01_part_02(input));

        assert_eq!(10, solve_day_01_part_02("R1000"));

        assert_eq!(
            1,
            solve_day_01_part_02(
                "
                       R30
                       R20
                       "
            )
        );

        assert_eq!(
            1,
            solve_day_01_part_02(
                "
                       L30
                       L20
                       "
            )
        );

        assert_eq!(1, solve_day_01_part_02("L60"));

        assert_eq!(1, solve_day_01_part_02("L50"));

        assert_eq!(3, solve_day_01_part_02("L250"));
    }
}
