const FIRST_NUMBER: usize = 20_151_125;
const MULTIPLIER: usize = 252_533;
const MODULO: usize = 33_554_393;

fn value_at(row: usize, column: usize) -> usize {
    let mut n = FIRST_NUMBER;
    let mut max_row = 1;
    let mut current_row = 0;
    let mut current_column = 0;

    loop {
        while current_row > 0 {
            n = (n * MULTIPLIER) % MODULO;

            if current_row == row && current_column == column {
                return n;
            }

            current_row -= 1;
            current_column += 1;
        }

        max_row += 1;
        current_row = max_row;
        current_column = 1;
    }
}

fn main() {
    let code = value_at(2978, 3083);

    dbg!(code);
}
