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

    people.insert("Me");

    for person in &people {
        relations.insert(("Me", *person), 0);
        relations.insert((*person, "Me"), 0);
    }

    let persons = people.clone();

    let max = people
        .into_iter()
        .permutations(persons.len())
        .map(|permutation| {
            let people_vec = permutation.clone();

            permutation
                .iter()
                .enumerate()
                .map(|(idx, &person)| {
                    let left = if idx == 0 {
                        people_vec[persons.len() - 1]
                    } else {
                        people_vec[idx - 1]
                    };

                    let right = if idx == persons.len() - 1 {
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
