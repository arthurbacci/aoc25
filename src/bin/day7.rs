use std::fs::File;
use std::io::Read;

use criterion::Criterion;

peg::parser! {
    grammar manifold_parser() for str {
        rule first_beam() -> usize
            = ("." *) p:position!() "S" ("." *) "\n" { p }

        rule splitter() -> usize
            = p:position!() "^" ("." *) { p }

        rule empty_line() -> Vec<usize>
            = ("." +) ("\n" ?) { vec![] }

        rule splitter_line() -> Vec<usize>
            = p:position!() ("." *) l:(splitter() +) ("\n" ?) {
                l.into_iter().map(|x| x - p).collect()
            }

        pub rule manifold() -> (usize, Vec<Vec<usize>>)
            = f:first_beam() m:((splitter_line() / empty_line()) +) ("\n" *) {
                (f, m)
            }
    }
}

fn part1() -> String {
    let mut f = File::open("day7.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let (beam, manifold) = manifold_parser::manifold(&data).unwrap();
    
    let mut beams = vec![beam];
    let mut total = 0;

    for ln in manifold {
        let mut next_beams = Vec::new();

        for r in ln.iter().filter(|x| beams.contains(x)) {
            if !next_beams.contains(&(r - 1)) && !beams.contains(&(r - 1)) {
                next_beams.push(r - 1);
            }
            next_beams.push(r + 1);
            total += 1;
        }

        beams = beams.into_iter()
            .filter(|x| !ln.contains(x))
            .chain(next_beams.into_iter())
            .collect();
    }

    total.to_string()
}

fn main() {
    println!("Part 1: {}", part1());
    // println!("Part 2: {}", part2());

    let mut c = Criterion::default();

    // c.bench_function("day7_part1", |b| b.iter(|| part1()));

    // c.bench_function("day7_part2", |b| b.iter(|| part2()));
}

