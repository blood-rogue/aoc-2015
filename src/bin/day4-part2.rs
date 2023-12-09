use md5::{Digest, Md5};

fn main() {
    let input = include_str!(r"..\..\input\day4.txt").trim();
    let mut i = 1;

    loop {
        let digest = Md5::digest(format!("{input}{i}").as_str());

        if matches!(digest.as_slice()[0..3], [0, 0, 0]) {
            dbg!(i);
            break;
        }

        i += 1;
    }
}
