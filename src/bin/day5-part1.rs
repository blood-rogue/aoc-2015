fn main() {
    let mut nice_count = 0;

    include_str!(r"..\..\input\day5.txt")
        .lines()
        .for_each(|line| {
            let mut prev_char = '\0';
            let mut vowel_count = 0;
            let mut contains_illegal = false;
            let mut contains_double = false;

            for ch in line.chars() {
                if matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u') {
                    vowel_count += 1;
                }

                if prev_char == ch {
                    contains_double = true;
                }

                if matches!(
                    format!("{prev_char}{ch}").as_str(),
                    "ab" | "cd" | "pq" | "xy"
                ) {
                    contains_illegal = true;
                }

                prev_char = ch;
            }

            if vowel_count >= 3 && !contains_illegal && contains_double {
                nice_count += 1;
            }
        });

    dbg!(nice_count);
}
