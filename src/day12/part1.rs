pub fn solve_day_12_part_01(input: &str) -> u32 {
    let mut tiles = Vec::new();
    let mut remaining_input = input;
    while let Some(next_end) = remaining_input.find("\n\n") {
        let tile_str = &remaining_input[..next_end];
        let tile = Tile::from(tile_str);
        tiles.push(tile);
        remaining_input = &remaining_input[next_end + 2..];
    }

    let mut count = 0;

    for (line_index, line) in remaining_input.lines().enumerate() {
        let (size, rest) = line.split_once(": ").unwrap();
        let (width, height) = size.split_once('x').unwrap();
        let width: usize = width.parse().unwrap();
        let height: usize = height.parse().unwrap();

        let counts: Vec<usize> = rest.split(' ').map(|part| part.parse().unwrap()).collect();

        // Trivially allowed: all tiles fit into their own 3x3 cell
        let tiles_allowed = (width / 3) * (height / 3);
        let total_tiles_requested: usize = counts.iter().sum();
        if total_tiles_requested <= tiles_allowed {
            count += 1;
            continue;
        }

        // Trivially impossible: not enough cells to hold the tiles no matter what
        let total_hashes_requested: usize = counts
            .iter()
            .zip(tiles.iter())
            .map(|(&c, t)| c * t.data.iter().filter(|&&b| b).count())
            .sum();
        let total_hashes_possible = width * height;
        if total_hashes_requested > total_hashes_possible {
            continue;
        }

        panic!("Required non-trivial check for line {line_index}");
    }

    count
}

struct Tile {
    data: Vec<bool>,
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let mut data = Vec::new();

        for line in value.lines() {
            if line.trim().is_empty() {
                continue;
            }

            for c in line.chars() {
                data.push(c == '#');
            }
        }

        Tile { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_12_part_01() {
        let input = read_string("./src/day12/input.txt").unwrap();

        let solution = solve_day_12_part_01(&input);

        assert_eq!(544, solution);
    }
}
