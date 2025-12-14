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
        let (to_split_beams, next_beams) = beams.into_iter()
            .partition(|x| ln.contains(x));
        beams = next_beams;

        for r in to_split_beams {
            if !beams.contains(&(r - 1)) {
                beams.push(r - 1);
            }
            if !beams.contains(&(r + 1)) {
                beams.push(r + 1);
            }
            total += 1;
        }
    }

    total.to_string()
}

fn part2() -> String {
    let mut f = File::open("day7.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let (beam, manifold) = manifold_parser::manifold(&data).unwrap();
    
    let mut beams = vec![(beam, 1)];
    let mut total = 0;

    for ln in manifold {
        let (to_split_beams, next_beams) = beams.into_iter()
            .partition(|(x, _)| ln.contains(x));
        beams = next_beams;

        for (r, t) in to_split_beams {
            if let Some(pos) = beams.iter().position(|&(x, _)| x == r - 1) {
                beams[pos].1 += t;
            } else {
                beams.push((r - 1, t));
            }
            if let Some(pos) = beams.iter().position(|&(x, _)| x == r + 1) {
                beams[pos].1 += t;
            } else {
                beams.push((r + 1, t));
            }
        }
    }

    beams.into_iter().map(|(_, t)| t).sum::<usize>().to_string()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

    let mut c = Criterion::default();

    // c.bench_function("day7_part1", |b| b.iter(|| part1()));

    // c.bench_function("day7_part2", |b| b.iter(|| part2()));
}

