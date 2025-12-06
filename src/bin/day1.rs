use std::fs::File;
use std::io::Read;

use criterion::Criterion;

peg::parser! {
    grammar rotations_parser() for str {
        rule dir_sign() -> i32
            = d:$("R" / "L") { if d == "R" { 1 } else { -1 } }

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule rotation() -> i32
            = d:(dir_sign()) n:(number()) { d * (n as i32) }

        rule rotation_line() -> i32
            = r:(rotation()) "\n" { r }

        pub rule rotations() -> Vec<i32>
            = l:(rotation_line() *) ("\n" *) { l }
    }
}

fn part1() -> String {
    let mut f = File::open("day1.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let rotations = rotations_parser::rotations(&data).unwrap();

    let mut dial = 50;
    let mut pass1 = 0;

    for rotation in rotations {
        dial = (dial + (rotation % 100) + 100) % 100;

        if dial == 0 {
            pass1 += 1;
        }
    }

    pass1.to_string()
}

fn part2() -> String {
    let mut f = File::open("day1.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let rotations = rotations_parser::rotations(&data).unwrap();

    let mut dial = 50;
    let mut pass2 = 0;

    for rotation in rotations {
        let rm = rotation % 100;

        pass2 += rotation.abs() / 100;

        dial += rm;

        if dial >= 100 || (dial < 0 && dial != rm) || dial == 0 {
            pass2 += 1;
        }

        dial += 100;
        dial %= 100;
    }

    pass2.to_string()
}

fn main() {
    let mut c = Criterion::default();

    c.bench_function("day1_part1", |b| b.iter(|| part1()));

    c.bench_function("day1_part2", |b| b.iter(|| part2()));
}

