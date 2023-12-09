fn main() {
    let sum = include_str!(r"..\..\input\day8.txt")
        .lines()
        .map(|line| {
            let mut parsed_len = line.len() + 2;

            for ch in line.chars() {
                match ch {
                    '\\' | '"' => parsed_len += 1,
                    _ => {}
                }
            }

            parsed_len - line.len()
        })
        .sum::<usize>();

    dbg!(sum);
}
