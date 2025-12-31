pub fn solve_day_09_part_02(input: &str) -> u32 {
    let vertices: Vec<(i32, i32)> = input
        .trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    // Coordinate compression: use only vertex coordinates
    use std::collections::{BTreeSet, HashMap, HashSet};

    let x_coords: BTreeSet<i32> = vertices.iter().map(|&(x, _)| x).collect();
    let y_coords: BTreeSet<i32> = vertices.iter().map(|&(_, y)| y).collect();

    let x_list: Vec<i32> = x_coords.into_iter().collect();
    let y_list: Vec<i32> = y_coords.into_iter().collect();

    let x_to_idx: HashMap<i32, usize> = x_list.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_to_idx: HashMap<i32, usize> = y_list.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    // Mark all tiles that are on edges or interior
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    for yi in 0..y_list.len() {
        for xi in 0..x_list.len() {
            let x = x_list[xi];
            let y = y_list[yi];

            // Check if on boundary or inside
            if is_on_edge(&vertices, x, y) || is_inside(&vertices, x, y) {
                tiles.insert((xi, yi));
            }
        }
    }

    // Build 2D prefix sum in compressed space
    let width = x_list.len();
    let height = y_list.len();
    let mut prefix = vec![vec![0i64; width + 1]; height + 1];

    for yi in 0..height {
        for xi in 0..width {
            let val = if tiles.contains(&(xi, yi)) { 1 } else { 0 };
            prefix[yi + 1][xi + 1] = val + prefix[yi + 1][xi] + prefix[yi][xi + 1] - prefix[yi][xi];
        }
    }

    // Helper to check if rectangle is fully filled
    let is_filled = |xi1: usize, yi1: usize, xi2: usize, yi2: usize| -> bool {
        let count = prefix[yi2 + 1][xi2 + 1] - prefix[yi1][xi2 + 1] - prefix[yi2 + 1][xi1]
            + prefix[yi1][xi1];
        let expected = ((xi2 - xi1 + 1) * (yi2 - yi1 + 1)) as i64;
        count == expected
    };

    // Find largest rectangle with vertices as opposite corners
    let mut max_area = 0i64;

    for i in 0..vertices.len() {
        for j in i + 1..vertices.len() {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[j];

            // Must be opposite corners (different x and y)
            if x1 == x2 || y1 == y2 {
                continue;
            }

            let xi1 = x_to_idx[&x1];
            let xi2 = x_to_idx[&x2];
            let yi1 = y_to_idx[&y1];
            let yi2 = y_to_idx[&y2];

            let (xi_min, xi_max) = if xi1 < xi2 { (xi1, xi2) } else { (xi2, xi1) };
            let (yi_min, yi_max) = if yi1 < yi2 { (yi1, yi2) } else { (yi2, yi1) };

            if is_filled(xi_min, yi_min, xi_max, yi_max) {
                // Calculate actual area using real coordinates
                let actual_width = (x_list[xi_max] - x_list[xi_min] + 1) as i64;
                let actual_height = (y_list[yi_max] - y_list[yi_min] + 1) as i64;
                let area = actual_width * actual_height;
                max_area = max_area.max(area);
            }
        }
    }

    max_area as u32
}

fn is_on_edge(vertices: &[(i32, i32)], x: i32, y: i32) -> bool {
    for i in 0..vertices.len() {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % vertices.len()];

        // Check if point is on this edge
        if x1 == x2 && x == x1 {
            let (y_min, y_max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            if y >= y_min && y <= y_max {
                return true;
            }
        } else if y1 == y2 && y == y1 {
            let (x_min, x_max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            if x >= x_min && x <= x_max {
                return true;
            }
        }
    }
    false
}

fn is_inside(vertices: &[(i32, i32)], x: i32, y: i32) -> bool {
    let mut inside = false;
    let n = vertices.len();

    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];

        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }

    inside
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;
    use indoc::indoc;

    #[test]
    fn should_solve_day_09_part_02() {
        let input = read_string("./src/day09/input.txt").unwrap();

        let solution = solve_day_09_part_02(&input);

        assert_eq!(1566346198, solution);
    }

    #[test]
    fn should_solve_day_09_part_02_sample() {
        let input = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "}
        .trim();

        assert_eq!(24, solve_day_09_part_02(input));
    }
}
