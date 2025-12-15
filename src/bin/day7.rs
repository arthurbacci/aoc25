use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};

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

fn parsed() -> (usize, Vec<Vec<usize>>) {
    let mut f = File::open("day7.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    manifold_parser::manifold(&data).unwrap()
}

fn part1() -> String {
    let (beam, manifold) = parsed();
    
    let mut beams = HashSet::new();
    beams.insert(beam);

    let mut total = 0;

    for ln in manifold {
        for r in ln {
            if beams.remove(&r) {
                beams.insert(r - 1);
                beams.insert(r + 1);

                total += 1;
            }
        }
    }

    total.to_string()
}

fn part2() -> String {
    let (beam, manifold) = parsed();
    
    let mut beams = HashMap::new();
    beams.insert(beam, 1);

    for ln in manifold {
        for r in ln {
            if let Some(t) = beams.remove(&r) {
                *beams.entry(r - 1).or_insert(0) += t;
                *beams.entry(r + 1).or_insert(0) += t;
            }
        }
    }

    beams.values().sum::<usize>().to_string()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

    let mut c = Criterion::default();

    c.bench_function("day7_part1", |b| b.iter(|| part1()));

    c.bench_function("day7_part2", |b| b.iter(|| part2()));

    c.bench_function("parser", |b| b.iter(|| parsed()));
}

