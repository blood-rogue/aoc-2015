use itertools::Itertools;

struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,
}

const DURATION: usize = 2503;

fn main() {
    let pruned = include_str!(r"..\..\input\day14.txt")
        .replace("can fly ", "")
        .replace(" km/s for", "")
        .replace(" seconds", "")
        .replace(", but then must rest for", "")
        .replace('.', "");

    let max = pruned
        .lines()
        .map(|line| {
            let (_, speed, duration, rest) = line.split(' ').collect_tuple().unwrap();

            Reindeer {
                speed: speed.parse().unwrap(),
                duration: duration.parse().unwrap(),
                rest: rest.parse().expect(rest),
            }
        })
        .map(|stats| {
            let distance =
                stats.speed * stats.duration * (DURATION / (stats.duration + stats.rest));

            if (DURATION % (stats.duration + stats.rest)) >= stats.duration {
                distance + stats.speed * stats.duration
            } else {
                distance + stats.speed * (DURATION % (stats.duration + stats.rest))
            }
        })
        .max()
        .unwrap();

    dbg!(max);
}
