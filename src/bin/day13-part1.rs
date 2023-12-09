use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let mut people = HashSet::new();
    let mut relations = HashMap::new();

    let pruned = include_str!(r"..\..\input\day13.txt")
        .trim()
        .replace("happiness units by sitting next to ", "")
        .replace("would ", "")
        .replace("gain ", "")
        .replace("lose ", "-")
        .replace('.', "");

    pruned.lines().for_each(|line| {
        let (from, value, source) = line.split(' ').collect_tuple().unwrap();

        let value = value.parse::<isize>().unwrap();

        people.insert(from);
        relations.insert((from, source), value);
    });

    let max = people
        .into_iter()
        .permutations(8)
        .map(|permutation| {
            let people_vec = permutation.clone();

            permutation
                .iter()
                .enumerate()
                .map(|(idx, &person)| {
                    let left = if idx == 0 {
                        people_vec[7]
                    } else {
                        people_vec[idx - 1]
                    };

                    let right = if idx == 7 {
                        people_vec[0]
                    } else {
                        people_vec[idx + 1]
                    };

                    relations[&(person, left)] + relations[&(person, right)]
                })
                .sum::<isize>()
        })
        .max()
        .unwrap();

    dbg!(max);
}
