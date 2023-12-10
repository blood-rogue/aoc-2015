use itertools::Itertools;

fn main() {
    let mut grid: [bool; 100 * 100] = include_str!(r"..\..\input\day18.txt")
        .lines()
        .flat_map(|line| line.bytes().map(|b| b == b'#').collect_vec())
        .collect_vec()
        .try_into()
        .unwrap();

    grid[0] = true;
    grid[99] = true;
    grid[9900] = true;
    grid[9999] = true;

    for _ in 0..100 {
        grid = change_state(&grid);
    }

    let count = grid.iter().filter(|&&b| b).count();

    dbg!(count);
}

fn change_state(grid: &[bool; 100 * 100]) -> [bool; 100 * 100] {
    let mut new_grid = *grid;

    for (row, col) in (0..100).cartesian_product(0..100) {
        let neighbours = match (row, col) {
            (0 | 99, 0 | 99) => vec![true, true, true],
            (0, _) => vec![
                grid[col + 1],
                grid[col - 1],
                grid[100 + (col - 1)],
                grid[100 + (col)],
                grid[100 + (col + 1)],
            ],
            (_, 0) => vec![
                grid[(row + 1) * 100],
                grid[(row - 1) * 100],
                grid[(row - 1) * 100 + (1)],
                grid[(row) * 100 + (1)],
                grid[(row + 1) * 100 + (1)],
            ],
            (99, _) => vec![
                grid[(99) * 100 + (col + 1)],
                grid[(99) * 100 + (col - 1)],
                grid[(98) * 100 + (col - 1)],
                grid[(98) * 100 + (col)],
                grid[(98) * 100 + (col + 1)],
            ],
            (_, 99) => vec![
                grid[(row + 1) * 100 + (99)],
                grid[(row - 1) * 100 + (99)],
                grid[(row - 1) * 100 + (98)],
                grid[(row) * 100 + (98)],
                grid[(row + 1) * 100 + (98)],
            ],
            _ => vec![
                grid[(row + 1) * 100 + (col - 1)],
                grid[(row + 1) * 100 + (col)],
                grid[(row + 1) * 100 + (col + 1)],
                grid[(row - 1) * 100 + (col - 1)],
                grid[(row - 1) * 100 + (col)],
                grid[(row - 1) * 100 + (col + 1)],
                grid[(row) * 100 + (col + 1)],
                grid[(row) * 100 + (col - 1)],
            ],
        };

        let on_count = neighbours.iter().filter(|&&b| b).count();

        if grid[(row) * 100 + (col)] && on_count != 2 && on_count != 3 {
            new_grid[(row) * 100 + (col)] = false;
        } else if !grid[(row) * 100 + (col)] && on_count == 3 {
            new_grid[(row) * 100 + (col)] = true;
        }
    }

    new_grid
}
