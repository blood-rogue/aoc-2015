use itertools::Itertools;

fn main() {
    let mut input = include_str!(r"..\..\input\day10.txt").trim().to_string();

    for _ in 0..50 {
        let mut prev_char = *input.chars().collect_vec().first().unwrap();
        let mut ch_count = 0u32;
        let mut counts = Vec::new();

        for ch in input.chars() {
            if prev_char == ch {
                ch_count += 1;
            } else {
                counts.push([char::from_digit(ch_count, 10).unwrap(), prev_char]);
                ch_count = 1;
            }

            prev_char = ch;
        }

        counts.push([char::from_digit(ch_count, 10).unwrap(), prev_char]);

        input = counts.iter().flatten().collect::<String>();
    }

    let len = input.len();

    dbg!(len);
}
