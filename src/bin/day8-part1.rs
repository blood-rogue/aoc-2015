fn main() {
    let sum = include_str!(r"..\..\input\day8.txt")
        .lines()
        .map(|line| {
            let mut parsed_len = line.len() - 2;
            let mut chars = line.chars();

            chars.next();

            loop {
                let Some(ch) = chars.next() else {
                    break;
                };

                if ch == '\\' {
                    let curr_ch = chars.next().unwrap();

                    if curr_ch == 'x' {
                        parsed_len -= 2;
                    }

                    parsed_len -= 1;
                }
            }

            line.len() - parsed_len
        })
        .sum::<usize>();

    dbg!(sum);
}
