use crate::day11::part1::parse;
use std::collections::HashMap;

pub fn solve_day_11_part_02(input: &str) -> u64 {
    let map = parse(input);
    let mut memo = HashMap::new();
    count_paths(&map, &mut memo, "svr", false, false)
}

fn count_paths(
    map: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), u64>,
    node: &str,
    mut found_dac: bool,
    mut found_fft: bool,
) -> u64 {
    let key = (node.to_string(), found_dac, found_fft);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if node == "out" {
        return if found_dac && found_fft { 1 } else { 0 };
    }

    let Some(neighbors) = map.get(node) else {
        return 0;
    };

    let mut total = 0;
    for neighbor in neighbors {
        match neighbor.as_str() {
            "dac" => found_dac = true,
            "fft" => found_fft = true,
            _ => {}
        }

        total += count_paths(map, memo, neighbor, found_dac, found_fft);
    }

    memo.insert(key, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_11_part_02() {
        let input = read_string("./src/day11/input.txt").unwrap();

        let solution = solve_day_11_part_02(&input);

        assert_eq!(557332758684000, solution);
    }

    #[test]
    fn should_solve_day_11_part_02_sample() {
        let input = indoc! {"
            svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out
        "}
        .trim();

        assert_eq!(2, solve_day_11_part_02(input));
    }
}
