use itertools::Itertools;

fn main() {
    let sum = include_str!(r"..\..\input\day2.txt")
        .lines()
        .map(|line| {
            let (l, w, h) = line
                .split('x')
                .map(|part| part.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap();

            find_dimensions(l * w, w * h, h * l)
        })
        .sum::<usize>();

    dbg!(sum);
}

const fn find_dimensions(a: usize, b: usize, c: usize) -> usize {
    2 * (a + b + c) + min(a, b, c)
}

const fn min(a: usize, b: usize, c: usize) -> usize {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}
