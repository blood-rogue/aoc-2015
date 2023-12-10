use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let curr_sue = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let sues = include_str!(r"..\..\input\day16.txt")
        .lines()
        .map(|line| {
            let (id, details) = line.split_once(": ").unwrap();

            (
                id,
                details
                    .split(", ")
                    .map(|detail| {
                        detail
                            .split_once(": ")
                            .map(|(s1, s2)| (s1, s2.parse::<u8>().unwrap()))
                            .unwrap()
                    })
                    .collect_vec(),
            )
        })
        .collect_vec();

    for (id, sue) in sues {
        if sue.iter().all(|&(detail, value)| {
            if let Some(&v) = curr_sue.get(detail) {
                match detail {
                    "cats" | "trees" => v < value,
                    "goldfish" | "pomeranians" => v > value,
                    _ => v == value,
                }
            } else {
                false
            }
        }) {
            dbg!(id);
            return;
        }
    }
}
