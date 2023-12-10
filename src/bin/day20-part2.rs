fn main() {
    let gifts = include_str!(r"..\..\input\day20.txt")
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut house = 1;

    loop {
        let mut factors = vec![];

        for f in 1..=(f64::from(house).sqrt() as u32) {
            if house % f == 0 {
                factors.push(f);
                factors.push(house / f);
            }
        }

        let factor_sum = factors.iter().fold(0, |acc, factor| {
            if house / *factor <= 50 {
                acc + factor * 11
            } else {
                acc
            }
        });

        if factor_sum > gifts {
            dbg!(house);
            break;
        }

        house += 1;
    }
}
