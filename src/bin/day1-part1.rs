fn main() {
    let floor = include_str!(r"..\..\input\day1.txt")
        .chars()
        .fold(0, |acc, ch| if ch == '(' { acc + 1 } else { acc - 1 });

    dbg!(floor);
}
