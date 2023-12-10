use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let (mapping, molecule) = include_str!(r"..\..\input\day19.txt")
        .trim()
        .split_once("\n\n")
        .unwrap();

    let mut all_molecules = HashSet::new();

    mapping.lines().for_each(|line| {
        let (from, to) = line.split_once(" => ").unwrap();

        let sub_molecules = molecule
            .split_inclusive(from)
            .map(ToString::to_string)
            .collect_vec();

        for (i, sub_mol) in sub_molecules.iter().enumerate() {
            let mut generated_molecule = sub_molecules.clone();
            generated_molecule[i] = sub_mol.replace(from, to);
            all_molecules.insert(generated_molecule.concat());
        }
    });

    all_molecules.remove(molecule);

    let count = all_molecules.len();
    dbg!(count);
}
