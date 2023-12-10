use itertools::Itertools;

fn main() {
    let containers = include_str!(r"..\..\input\day17.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();

    let count = (3..=containers.len())
        .flat_map(|len| containers.iter().copied().combinations(len).collect_vec())
        .filter(|combination| combination.iter().sum::<usize>() == 150)
        .count();

    dbg!(count);
}
