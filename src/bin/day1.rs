use std::fs::File;
use std::io::Read;

use bare_metal_modulo::WrapCountNumC;

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

fn main() {
    let mut f = File::open("day1.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let rotations = rotations_parser::rotations(&data).unwrap();

    let mut dial: WrapCountNumC<i32, 100> = WrapCountNumC::new(50);
    let mut pass1 = 0;
    let mut pass2 = 0;

    for rotation in rotations {

        if dial == 0 && rotation < 0 {
            pass2 -= 1;
        }

        dial = dial + rotation;

        pass2 += dial.wraps().abs();

        if dial == 0 && rotation < 0 {
            pass2 += 1;
        }

        if dial == 0 {
            pass1 += 1;
        }
    }

    println!("Part 1 password: {}", pass1);
    println!("Part 2 password: {}", pass2);
}
