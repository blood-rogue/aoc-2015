use std::collections::HashSet;

fn main() {
    let mut map = HashSet::new();
    let mut last_loc = (0, 0);

    map.insert(last_loc);

    include_str!(r"..\..\input\day3.txt")
        .chars()
        .for_each(|ch| {
            match ch {
                '^' => last_loc.0 += 1,
                'v' => last_loc.0 -= 1,
                '>' => last_loc.1 += 1,
                '<' => last_loc.1 -= 1,
                _ => {}
            }
            map.insert(last_loc);
        });

    let count = map.len();

    dbg!(count);
}
