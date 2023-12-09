use itertools::Itertools;

fn main() {
    let mut password: [u8; 8] = include_str!(r"..\..\input\day11.txt")
        .trim()
        .bytes()
        .map(|byte| byte - b'a')
        .collect_vec()
        .try_into()
        .unwrap();

    loop {
        password = next_password(password);

        if check(password) {
            break;
        }
    }

    loop {
        password = next_password(password);

        if check(password) {
            break;
        }
    }

    let password = password
        .iter()
        .map(|ch| char::from(*ch + b'a'))
        .collect::<String>();

    dbg!(password);
}

const fn next_password(curr: [u8; 8]) -> [u8; 8] {
    let mut next = curr;

    let mut idx = 7;
    let mut cont = true;

    while cont {
        if next[idx] == 25 {
            next[idx] = 0;
            idx -= 1;
        } else {
            next[idx] += 1;
            cont = false;
        }
    }

    next
}

fn check(password: [u8; 8]) -> bool {
    if [8, 11, 14].iter().any(|ch| password.contains(ch)) {
        return false;
    }

    if !password
        .windows(3)
        .any(|window| window[1] == window[0] + 1 && window[2] == window[1] + 1)
    {
        return false;
    }

    let mut pair_count = 0;
    let mut prev_char = Some(password[0]);

    for &ch in password.iter().skip(1) {
        if prev_char == Some(ch) {
            pair_count += 1;
            prev_char = None;
        } else {
            prev_char = Some(ch);
        }
    }

    pair_count >= 2
}
