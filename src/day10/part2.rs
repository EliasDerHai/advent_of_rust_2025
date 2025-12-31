use crate::day10::part1::{parse, Machine};
use z3::ast::Int;
use z3::Optimize;

pub fn solve_day_10_part_02(input: &str) -> u32 {
    parse(input)
        .iter()
        .fold(0, |aggr, machine| machine.calibrate_joltage() + aggr)
}

impl Machine {
    fn calibrate_joltage(&self) -> u32 {
        let optimizer = Optimize::new();

        // Create integer variables for each button (number of presses)
        let button_vars: Vec<Int> = (0..self.button_groups.len())
            .map(|i| Int::new_const(i.to_string()))
            .collect();

        // Each button must be pressed >= 0 times
        for var in &button_vars {
            optimizer.assert(&var.ge(Int::from_i64(0)));
        }

        // For each joltage position, sum of button presses affecting it must equal target
        for (idx, &target) in self.joltage_req.iter().enumerate() {
            let mut terms: Vec<Int> = Vec::new();

            for (button_idx, button_group) in self.button_groups.iter().enumerate() {
                if button_group.contains(&(idx as u32)) {
                    terms.push(button_vars[button_idx].clone());
                }
            }

            if !terms.is_empty() {
                let sum = Int::add(&terms.iter().collect::<Vec<_>>());
                optimizer.assert(&sum.eq(Int::from_i64(target as i64)));
            } else {
                // If no button affects this position, target must be 0
                assert_eq!(
                    target, 0,
                    "No button affects position {idx} but target is {target}"
                );
            }
        }

        // Minimize total button presses
        let total = Int::add(&button_vars.iter().collect::<Vec<_>>());
        optimizer.minimize(&total);

        // Solve and extract result
        match optimizer.check(&[]) {
            z3::SatResult::Sat => {
                let model = optimizer.get_model().unwrap();
                let result = model.eval(&total, true).unwrap();
                result.as_i64().unwrap() as u32
            }
            _ => panic!("No solution found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_10_part_02() {
        let input = read_string("./src/day10/input.txt").unwrap();

        let solution = solve_day_10_part_02(&input);

        assert_eq!(20617, solution);
    }

    #[test]
    fn should_solve_day_10_part_02_sample() {
        let input = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "}
        .trim();

        assert_eq!(33, solve_day_10_part_02(input));
    }
}
