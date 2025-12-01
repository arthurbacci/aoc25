use std::fs::File;
use std::io::Read;

use bare_metal_modulo::ModNumC;

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
            = l:(rotation_line() *) { l }
    }
}

fn main() {
    let mut f = File::open("day1.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let rotations = rotations_parser::rotations(&data).unwrap();

    let mut dial: ModNumC<i32, 100> = ModNumC::new(50);
    let mut pass = 0;

    for rotation in rotations {
        dial += rotation;
        if dial == 0 {
            pass += 1;
        }
    }

    println!("Password: {}", pass);
}
