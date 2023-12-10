use itertools::Itertools;

fn main() {
    let (mapping, molecule) = include_str!(r"..\..\input\day19.txt")
        .trim()
        .split_once("\n\n")
        .unwrap();

    let mut molecule = molecule.to_string();

    let mapping = mapping
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect_vec();

    let mut iterations = 0usize;

    while molecule != "e" {
        for (from, to) in &mapping {
            if molecule.contains(to) {
                molecule = molecule.replacen(to, from, 1);
                iterations += 1;
            }
        }
    }

    dbg!(iterations);
}
