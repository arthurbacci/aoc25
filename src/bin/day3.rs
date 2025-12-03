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

    let mut part1 = 0;
    let mut part2 = 0;

    for bank in banks {
        for part in 1..=2 {
            let max: Box<dyn Iterator<Item=u64>> = Box::new(
                bank.iter()
                    .rev()
                    .scan(0, |st, &x| {
                        if x > *st {
                            *st = x;
                        }
                        Some(*st as u64)
                    })
            );

            let result = (1..(if part == 1 { 2 } else { 12 }))
                .fold(max, |max, it| {
                    Box::new(
                        bank.iter()
                            .rev()
                            .skip(it)
                            .zip(max)
                            .map(move |(n, m)|
                                (*n as u64) * 10_u64.pow(it as u32) + m
                            )
                            .scan(0, |st, x| {
                                if x > *st {
                                    *st = x;
                                }
                                Some(*st)
                            })
                    )
                })
                .last()
                .unwrap();

            if part == 1 {
                part1 += result;
            } else {
                part2 += result;
            }
        }
        
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

