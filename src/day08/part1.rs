use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
pub struct P3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl P3 {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}

pub fn solve_day_08_part_01(input: &str, connections: usize) -> u32 {
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

    let mut dists: Vec<((usize, usize), u64)> = Vec::new();
    let mut circs: Vec<HashSet<usize>> = Vec::new();

    for (idx_a, p_a) in &points {
        for (idx_b, p_b) in &points[*idx_a + 1..] {
            let dx = p_a.x.abs_diff(p_b.x) as u64;
            let dy = p_a.y.abs_diff(p_b.y) as u64;
            let dz = p_a.z.abs_diff(p_b.z) as u64;
            let dist = dx * dx + dy * dy + dz * dz; // sqrt not needed for sorting
            dists.push(((*idx_a, *idx_b), dist));
        }
    }

    for &((from, to), _) in &dists
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .collect::<Vec<_>>()[..connections]
    {
        let circ_from = circs.iter().position(|circ| circ.contains(&from));
        let circ_to = circs.iter().position(|circ| circ.contains(&to));

        match (circ_from, circ_to) {
            (None, None) => {
                circs.push({
                    let mut h = HashSet::new();
                    h.insert(from);
                    h.insert(to);
                    h
                });
            }
            (None, Some(circ_to)) => {
                circs[circ_to].insert(from);
            }
            (Some(circ_from), None) => {
                circs[circ_from].insert(to);
            }
            (Some(circ_from), Some(circ_to)) if circ_from == circ_to => {
                // nothing to do (already same circuit)
            }
            (Some(circ_from_idx), Some(circ_to_idx)) => {
                // removing 'lower' idx shifts position of 'higher' lookup/insert
                if circ_from_idx > circ_to_idx {
                    let circ_from = circs.remove(circ_from_idx);
                    circs[circ_to_idx].extend(circ_from);
                } else {
                    let circ_to = circs.remove(circ_to_idx);
                    circs[circ_from_idx].extend(circ_to);
                }
            }
        }
    }

    circs.sort_by(|a, b| Ord::cmp(&a.len(), &b.len()));
    let mut it = circs.iter().rev();
    (it.next().unwrap().len() * it.next().unwrap().len() * it.next().unwrap().len()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_08_part_01() {
        let input = read_string("./src/day08/input.txt").unwrap();

        let solution = solve_day_08_part_01(&input, 1000);

        assert_eq!(75582, solution);
    }

    #[test]
    fn should_solve_day_08_part_01_sample() {
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

        assert_eq!(40, solve_day_08_part_01(input, 10));
    }
}
