fn main() {
    let mut floor = 0;
    for (i, ch) in include_str!(r"..\..\input\day1.txt").chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }

        if floor == -1 {
            dbg!(i + 1);
            break;
        }
    }
}
