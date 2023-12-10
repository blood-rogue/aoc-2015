use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let curr_sue = HashMap::from([
        ("children", "3"),
        ("cats", "7"),
        ("samoyeds", "2"),
        ("pomeranians", "3"),
        ("akitas", "0"),
        ("vizslas", "0"),
        ("goldfish", "5"),
        ("trees", "3"),
        ("cars", "2"),
        ("perfumes", "1"),
    ]);

    let sues = include_str!(r"..\..\input\day16.txt")
        .lines()
        .map(|line| {
            let (id, details) = line.split_once(": ").unwrap();

            (
                id.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
                details
                    .split(", ")
                    .map(|detail| detail.split_once(": ").unwrap())
                    .collect_vec(),
            )
        })
        .collect::<HashMap<_, _>>();

    for (id, sue) in sues {
        if sue.iter().all(|&(detail, value)| {
            if let Some(&v) = curr_sue.get(detail) {
                v == value
            } else {
                false
            }
        }) {
            dbg!(id);
            return;
        }
    }
}
