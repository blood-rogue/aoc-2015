use itertools::Itertools;

fn main() {
    let mut lights = vec![false; 1000 * 1000];

    include_str!(r"..\..\input\day6.txt")
        .replace("turn on", "on")
        .replace("turn off", "off")
        .replace(" through ", " ")
        .lines()
        .for_each(|line| {
            let (action, start, stop) = line.split(' ').collect_tuple().unwrap();

            let (x_start, y_start) = start
                .split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap();

            let (x_stop, y_stop) = stop
                .split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap();

            let func = get_func(action);

            for x in x_start..=x_stop {
                for y in y_start..=y_stop {
                    lights[x * 1000 + y] = func(lights[x * 1000 + y]);
                }
            }
        });

    let count = lights.iter().filter(|light| **light).count();

    dbg!(count);
}

fn get_func(action: &str) -> impl Fn(bool) -> bool {
    match action {
        "on" => |_| true,
        "off" => |_| false,
        "toggle" => |b: bool| !b,
        _ => panic!(),
    }
}
