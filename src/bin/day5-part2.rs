fn main() {
    let mut nice_count = 0;

    include_str!(r"..\..\input\day5.txt")
        .lines()
        .for_each(|line| {
            let mut last_to_prev_char = '\0';
            let mut prev_char = '\0';
            let mut contains_separated = false;
            let mut contains_repeated_pair = false;

            for ch in line.chars() {
                if last_to_prev_char == ch {
                    contains_separated = true;
                }

                if line.matches(&format!("{prev_char}{ch}")).count() > 1 {
                    contains_repeated_pair = true;
                }

                last_to_prev_char = prev_char;
                prev_char = ch;
            }

            if contains_separated && contains_repeated_pair {
                nice_count += 1;
            }
        });

    dbg!(nice_count);
}
