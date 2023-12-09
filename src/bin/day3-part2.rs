use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let mut map = HashSet::new();
    let mut santa_last_loc = (0, 0);
    let mut robo_santa_last_loc = (0, 0);

    map.insert(santa_last_loc);

    include_str!(r"..\..\input\day3.txt")
        .chars()
        .collect_vec()
        .chunks_exact(2)
        .for_each(|chunk| {
            match chunk[0] {
                '^' => santa_last_loc.0 += 1,
                'v' => santa_last_loc.0 -= 1,
                '>' => santa_last_loc.1 += 1,
                '<' => santa_last_loc.1 -= 1,
                _ => {}
            }

            map.insert(santa_last_loc);

            match chunk[1] {
                '^' => robo_santa_last_loc.0 += 1,
                'v' => robo_santa_last_loc.0 -= 1,
                '>' => robo_santa_last_loc.1 += 1,
                '<' => robo_santa_last_loc.1 -= 1,
                _ => {}
            }
            map.insert(robo_santa_last_loc);
        });

    let count = map.len();

    dbg!(count);
}
