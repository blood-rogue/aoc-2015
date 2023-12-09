use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let mut places = HashSet::new();
    let mut distances = HashMap::new();

    include_str!(r"..\..\input\day9.txt")
        .lines()
        .for_each(|line| {
            let (dir, dist) = line.split_once(" = ").unwrap();

            let dist = dist.parse::<usize>().unwrap();

            let (start, stop) = dir.split_once(" to ").unwrap();

            places.insert(start);
            places.insert(stop);

            distances.insert((start, stop), dist);
            distances.insert((stop, start), dist);
        });

    let max = places
        .iter()
        .permutations(places.len())
        .map(|permutation| {
            let mut total = 0;

            for place in permutation.windows(2) {
                total += distances[&(*place[0], *place[1])];
            }

            total
        })
        .max()
        .unwrap();

    dbg!(max);
}
