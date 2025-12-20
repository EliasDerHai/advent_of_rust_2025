/// assumes 'rectangular' input (each row has same width)
pub fn rotate_clockwise_90(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();

    let mut out = Vec::with_capacity(w);
    for c in 0..w {
        let mut line = String::with_capacity(h);
        for r in (0..h).rev() {
            line.push(grid[r][c]);
        }
        out.push(line);
    }

    out.join("\n")
}
