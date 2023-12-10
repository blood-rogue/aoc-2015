#[derive(Clone, Copy, PartialEq, Eq)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    const fn new(cost: usize, damage: usize, armor: usize) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

struct Character {
    damage: usize,
    armor: usize,
    health: usize,
}

impl Character {
    const fn new(damage: usize, armor: usize, health: usize) -> Self {
        Self {
            damage,
            armor,
            health,
        }
    }

    const fn with(&self, bundle: Item) -> Self {
        Self {
            damage: bundle.damage,
            armor: bundle.armor,
            health: self.health,
        }
    }
}

const fn get_stats(weapon: Item, armor: Item, left_ring: Item, right_ring: Item) -> Item {
    Item::new(
        weapon.cost + armor.cost + left_ring.cost + right_ring.cost,
        weapon.damage + left_ring.damage + right_ring.damage,
        armor.armor + left_ring.armor + right_ring.armor,
    )
}

const PLAYER: Character = Character::new(0, 0, 100);
const BOSS: Character = Character::new(8, 2, 100);

const WEAPONS: [Item; 5] = [
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMORS: [Item; 6] = [
    Item::new(0, 0, 0),
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: [Item; 7] = [
    Item::new(0, 0, 0),
    Item::new(25, 1, 0),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
    Item::new(20, 0, 1),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
];

fn main() {
    let mut bundles = Vec::new();

    for &weapon in &WEAPONS {
        for &armor in &ARMORS {
            for &left_ring in &RINGS {
                for &right_ring in &RINGS {
                    if left_ring != right_ring {
                        bundles.push(get_stats(weapon, armor, left_ring, right_ring));
                    }
                }
            }
        }
    }

    let mut min_cost = usize::MAX;

    for bundle in bundles {
        let player = PLAYER.with(bundle);

        let boss_hps = BOSS.health / get_attack(player.damage, BOSS.armor);
        let player_hps = PLAYER.health / get_attack(BOSS.damage, player.armor);

        if boss_hps <= player_hps {
            min_cost = min_cost.min(bundle.cost);
        }
    }

    dbg!(min_cost);
}

const fn get_attack(damage: usize, armor: usize) -> usize {
    if damage > armor {
        damage - armor
    } else {
        1
    }
}
