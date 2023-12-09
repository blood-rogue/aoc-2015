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

            find_length(l, w, h)
        })
        .sum::<usize>();

    dbg!(sum);
}

const fn find_length(a: usize, b: usize, c: usize) -> usize {
    (a * b * c) + 2 * min(a + b, b + c, c + a)
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
