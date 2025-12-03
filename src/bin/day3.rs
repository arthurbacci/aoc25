use std::fs::File;
use std::io::Read;

peg::parser! {
    grammar banks_parser() for str {
        rule digit() -> u8
            = n:$(['0'..='9']) {? n.parse().or(Err("u8")) }

        rule bank() -> Vec<u8>
            = b:(digit() +) "\n" { b }

        pub rule banks() -> Vec<Vec<u8>>
            = b:(bank() +) ("\n" *) { b }
    }
}



fn main() {
    let mut f = File::open("day3.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let banks = banks_parser::banks(&data).unwrap();

    let mut part1_total: u32 = 0;

    for bank in banks {
        let (max_pos, max) = bank.iter()
            // Can't be the last one
            .take(bank.len() - 1)
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap();

        let max = bank.iter()
            .take(bank.len() - 1)
            .max()
            .unwrap();
        
        let max_positions = bank.iter()
            .enumerate()
            .take(bank.len() - 1)
            .filter(|(_, x)| *x == max);

        let second_max = max_positions
            .map(
                |(max_pos, _)| bank.iter()
                    .skip(max_pos + 1)
                    .max()
                    .unwrap()
            )
            .max()
            .unwrap();
        
        part1_total += (max * 10 + second_max) as u32;
    }

    println!("{part1_total}");
}
