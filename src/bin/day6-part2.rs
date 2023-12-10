use itertools::Itertools;

fn main() {
    let mut lights = vec![0usize; 1000 * 1000];

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

            for (x, y) in (x_start..=x_stop).cartesian_product(y_start..=y_stop) {
                lights[x * 1000 + y] = func(lights[x * 1000 + y]);
            }
        });

    let sum = lights.iter().sum::<usize>();

    dbg!(sum);
}

fn get_func(action: &str) -> impl Fn(usize) -> usize {
    match action {
        "on" => |i| i + 1,
        "off" => |i| if i == 0 { 0 } else { i - 1 },
        "toggle" => |i| i + 2,
        _ => panic!(),
    }
}
