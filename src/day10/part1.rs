use itertools::Itertools;

pub fn solve_day_10_part_01(input: &str) -> u32 {
    parse(input)
        .iter()
        .fold(0, |aggr, machine| machine.turn_on() + aggr)
}

#[derive(Debug)]
pub struct Machine {
    pub indicator_lights: Vec<bool>,
    pub button_groups: Vec<Vec<u32>>,
    pub joltage_req: Vec<u32>,
}

impl Machine {
    fn new(indicator_lights: Vec<bool>, buttons: Vec<Vec<u32>>, joltage_req: Vec<u32>) -> Self {
        Self {
            indicator_lights,
            button_groups: buttons,
            joltage_req,
        }
    }

    /// find the lowest amount of button_presses needed to flip machine on
    /// initially all indicator lights are off, in order to be on, the lights have to match self.indicator_lights
    fn turn_on(&self) -> u32 {
        for num_presses in 0..=self.button_groups.len() {
            let combinations = (0..self.button_groups.len()).combinations(num_presses);

            for combination in combinations {
                let mut state = vec![false; self.indicator_lights.len()];

                for button_idx in combination {
                    for &light in &self.button_groups[button_idx] {
                        state[light as usize] = !state[light as usize];
                    }
                }

                if state == self.indicator_lights {
                    return num_presses as u32;
                }
            }
        }

        unreachable!()
    }
}

pub fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (indicator_lights, tail) = line.trim().split_once("] (").unwrap();
            let indicator_lights = indicator_lights
                .chars()
                .skip(1)
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    o => panic!("unexpected ${o}"),
                })
                .collect::<Vec<_>>();

            let (buttons, tail) = tail.split_once(") {").unwrap();

            let buttons = buttons
                .split(") (")
                .map(|seq| {
                    seq.split(",")
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let joltage_req = tail[0..tail.len() - 1]
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            Machine::new(indicator_lights, buttons, joltage_req)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_10_part_01() {
        let input = read_string("./src/day10/input.txt").unwrap();

        let solution = solve_day_10_part_01(&input);

        assert_eq!(491, solution);
    }

    #[test]
    fn should_solve_day_10_part_01_sample() {
        let input = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "}
        .trim();

        assert_eq!(7, solve_day_10_part_01(input));
    }
}
