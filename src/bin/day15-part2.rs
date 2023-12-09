use itertools::Itertools;

#[derive(Clone, Copy)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize,
}

fn main() {
    let pruned = include_str!(r"..\..\input\day15.txt")
        .trim()
        .replace(": capacity", "")
        .replace(", durability", "")
        .replace(", flavor", "")
        .replace(", texture", "")
        .replace(", calories", "");

    let ingredients = pruned
        .lines()
        .map(|line| {
            let (_, capacity, durability, flavor, texture, calories) =
                line.split(' ').collect_tuple().unwrap();

            Ingredient {
                capacity: capacity.parse().unwrap(),
                durability: durability.parse().unwrap(),
                flavor: flavor.parse().unwrap(),
                texture: texture.parse().unwrap(),
                calories: calories.parse().unwrap(),
            }
        })
        .collect_vec();

    let max = (1..100)
        .permutations(4)
        .filter(|permutation| permutation.iter().sum::<isize>() == 100)
        .map(|permutation| {
            let calories = ingredients
                .iter()
                .zip(&permutation)
                .map(|(&ingredient, &permutation)| ingredient.calories * permutation.unsigned_abs())
                .sum::<usize>();

            if calories != 500 {
                return 0;
            }

            let capacity = ingredients
                .iter()
                .zip(&permutation)
                .map(|(&ingredient, &permutation)| ingredient.capacity * permutation)
                .sum::<isize>();
            let capacity = if capacity.is_negative() {
                0
            } else {
                capacity.unsigned_abs()
            };

            let durability = ingredients
                .iter()
                .zip(&permutation)
                .map(|(&ingredient, &permutation)| ingredient.durability * permutation)
                .sum::<isize>();
            let durability = if durability.is_negative() {
                0
            } else {
                durability.unsigned_abs()
            };

            let flavor = ingredients
                .iter()
                .zip(&permutation)
                .map(|(&ingredient, &permutation)| ingredient.flavor * permutation)
                .sum::<isize>();
            let flavor = if flavor.is_negative() {
                0
            } else {
                flavor.unsigned_abs()
            };

            let texture = ingredients
                .iter()
                .zip(&permutation)
                .map(|(&ingredient, &permutation)| ingredient.texture * permutation)
                .sum::<isize>();
            let texture = if texture.is_negative() {
                0
            } else {
                texture.unsigned_abs()
            };

            capacity * durability * flavor * texture
        })
        .max()
        .unwrap();

    dbg!(max);
}
