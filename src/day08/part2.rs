use crate::{day08::part1::P3, util::find_union::FindUnion};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn solve_day_08_part_02(input: &str) -> u32 {
    let points: Vec<(usize, P3)> = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut it = line.split(',').map(|v| v.parse::<u32>().unwrap());
            (
                id,
                P3::new(it.next().unwrap(), it.next().unwrap(), it.next().unwrap()),
            )
        })
        .collect();

    let mut heap = BinaryHeap::new();
    let mut fu = FindUnion::new(points.len());

    for (idx_a, p_a) in &points {
        for (idx_b, p_b) in &points[*idx_a + 1..] {
            let dx = p_a.x.abs_diff(p_b.x) as u64;
            let dy = p_a.y.abs_diff(p_b.y) as u64;
            let dz = p_a.z.abs_diff(p_b.z) as u64;
            let dist = dx * dx + dy * dy + dz * dz; // sqrt not needed for sorting
            heap.push(Reverse((dist, *idx_a, *idx_b)));
        }
    }

    while let Some(Reverse((_, from, to))) = heap.pop() {
        fu.union(from, to);

        if *fu.sizes.iter().max().expect("not empty") == fu.sizes.len() as u32 {
            return points[from].1.x * points[to].1.x;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_08_part_02() {
        let input = read_string("./src/day08/input.txt").unwrap();

        let solution = solve_day_08_part_02(&input);

        assert_eq!(59039696, solution);
    }

    #[test]
    fn should_solve_day_08_part_02_sample() {
        let input = indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "}
        .trim();

        assert_eq!(25272, solve_day_08_part_02(input));
    }
}
