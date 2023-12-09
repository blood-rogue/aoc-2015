use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,
    points: usize,
}

fn main() {
    let pruned = include_str!(r"..\..\input\day14.txt")
        .replace("can fly ", "")
        .replace(" km/s for", "")
        .replace(" seconds", "")
        .replace(", but then must rest for", "")
        .replace('.', "");

    let mut reindeers = pruned
        .lines()
        .map(|line| {
            let (_, speed, duration, rest) = line.split(' ').collect_tuple().unwrap();

            Reindeer {
                speed: speed.parse().unwrap(),
                duration: duration.parse().unwrap(),
                rest: rest.parse().expect(rest),
                points: 0,
            }
        })
        .collect_vec();

    for curr_duration in 0..2503 {
        reindeers
            .iter_mut()
            .map(|stats| {
                let distance =
                    stats.speed * stats.duration * (curr_duration / (stats.duration + stats.rest));

                let distance = if (curr_duration % (stats.duration + stats.rest)) >= stats.duration
                {
                    distance + stats.speed * stats.duration
                } else {
                    distance + stats.speed * (curr_duration % (stats.duration + stats.rest))
                };

                (stats, distance)
            })
            .max_by(|&(_, dist1), &(_, dist2)| dist1.cmp(&dist2))
            .unwrap()
            .0
            .points += 1;
    }

    let max = reindeers.iter().map(|stats| stats.points).max().unwrap();

    dbg!(max);
}
