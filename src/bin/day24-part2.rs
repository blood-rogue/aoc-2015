use itertools::Itertools;

fn main() {
    let weights = include_str!(r"..\..\input\day24.txt")
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();

    let total_sum = weights.iter().sum::<usize>();
    let group_weight = total_sum / 4;

    let mut valid_packages = Vec::new();

    let mut i = 1;

    while valid_packages.is_empty() {
        i += 1;

        let combinations = weights.iter().copied().combinations(i);

        for comb in combinations {
            if comb.iter().sum::<usize>() == group_weight {
                valid_packages.push(comb);
            }
        }

        i += 1;
    }

    let min_size = valid_packages.iter().map(Vec::len).min().unwrap();

    let min = valid_packages
        .iter()
        .filter_map(|package| {
            (package.len() == min_size).then_some(package.iter().product::<usize>())
        })
        .min()
        .unwrap();

    dbg!(min);
}
