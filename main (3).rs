#![allow(non_snake_case, non_camel_case_types, dead_code)]

pub fn polarity(board: &[&str], specs: &(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)) -> Vec<String> {
    let height = board.len();
    let width = board[0].len();

    let mut grid: Vec<Vec<char>> = vec![vec!['X'; width]; height];
    let board: Vec<Vec<char>> = board.iter().map(|row| row.chars().collect()).collect();
    let (left, right, top, bottom) = specs;

    let mut tiles = vec![];
    let mut visited = vec![vec![false; width]; height];

    for r in 0..height {
        for c in 0..width {
            if visited[r][c] { continue; }
            match board[r][c] {
                'L' => {
                    tiles.push(((r, c), (r, c + 1)));
                    visited[r][c] = true;
                    visited[r][c + 1] = true;
                }
                'T' => {
                    tiles.push(((r, c), (r + 1, c)));
                    visited[r][c] = true;
                    visited[r + 1][c] = true;
                }
                _ => {}
            }
        }
    }

    let mut plus_row = vec![0; height];
    let mut minus_row = vec![0; height];
    let mut plus_col = vec![0; width];
    let mut minus_col = vec![0; width];

    fn is_safe(grid: &Vec<Vec<char>>, r: usize, c: usize, ch: char) -> bool {
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);
            if let Some(row) = grid.get(nr) {
                if let Some(&val) = row.get(nc) {
                    if val == ch {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn fits(val: i32, constraint: i32) -> bool {
        constraint == -1 || val <= constraint
    }

    fn exact(val: i32, constraint: i32) -> bool {
        constraint == -1 || val == constraint
    }

    fn solve(
        idx: usize,
        tiles: &Vec<((usize, usize), (usize, usize))>,
        grid: &mut Vec<Vec<char>>,
        plus_row: &mut Vec<i32>,
        minus_row: &mut Vec<i32>,
        plus_col: &mut Vec<i32>,
        minus_col: &mut Vec<i32>,
        left: &Vec<i32>,
        right: &Vec<i32>,
        top: &Vec<i32>,
        bottom: &Vec<i32>,
    ) -> bool {
        if idx == tiles.len() {
            return (0..plus_row.len()).all(|i| exact(plus_row[i], left[i]) && exact(minus_row[i], right[i]))
                && (0..plus_col.len()).all(|i| exact(plus_col[i], top[i]) && exact(minus_col[i], bottom[i]));
        }

        let ((r1, c1), (r2, c2)) = tiles[idx];
        let choices = [('+', '-'), ('-', '+'), ('X', 'X')];

        for &(ch1, ch2) in &choices {
            if ch1 != 'X' {
                if !is_safe(grid, r1, c1, ch1) || !is_safe(grid, r2, c2, ch2) {
                    continue;
                }
            }

            // Save state before placement
            let pr1 = plus_row[r1]; let pr2 = plus_row[r2];
            let mr1 = minus_row[r1]; let mr2 = minus_row[r2];
            let pc1 = plus_col[c1]; let pc2 = plus_col[c2];
            let mc1 = minus_col[c1]; let mc2 = minus_col[c2];

            match ch1 {
                '+' => { plus_row[r1] += 1; plus_col[c1] += 1; }
                '-' => { minus_row[r1] += 1; minus_col[c1] += 1; }
                _ => {}
            }
            match ch2 {
                '+' => { plus_row[r2] += 1; plus_col[c2] += 1; }
                '-' => { minus_row[r2] += 1; minus_col[c2] += 1; }
                _ => {}
            }

            if ch1 != 'X' && (
                !fits(plus_row[r1], left[r1]) || !fits(minus_row[r1], right[r1]) ||
                !fits(plus_row[r2], left[r2]) || !fits(minus_row[r2], right[r2]) ||
                !fits(plus_col[c1], top[c1]) || !fits(minus_col[c1], bottom[c1]) ||
                !fits(plus_col[c2], top[c2]) || !fits(minus_col[c2], bottom[c2])
            ) {
                plus_row[r1] = pr1; plus_row[r2] = pr2;
                minus_row[r1] = mr1; minus_row[r2] = mr2;
                plus_col[c1] = pc1; plus_col[c2] = pc2;
                minus_col[c1] = mc1; minus_col[c2] = mc2;
                continue;
            }

            grid[r1][c1] = ch1;
            grid[r2][c2] = ch2;

            if solve(idx + 1, tiles, grid, plus_row, minus_row, plus_col, minus_col, left, right, top, bottom) {
                return true;
            }

            grid[r1][c1] = 'X';
            grid[r2][c2] = 'X';
            plus_row[r1] = pr1; plus_row[r2] = pr2;
            minus_row[r1] = mr1; minus_row[r2] = mr2;
            plus_col[c1] = pc1; plus_col[c2] = pc2;
            minus_col[c1] = mc1; minus_col[c2] = mc2;
        }

        false
    }

    solve(0, &tiles, &mut grid, &mut plus_row, &mut minus_row, &mut plus_col, &mut minus_col, left, right, top, bottom);

    grid.into_iter().map(|row| row.into_iter().collect()).collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;




