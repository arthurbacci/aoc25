use std::fs::File;
use std::io::Read;

use criterion::Criterion;

peg::parser! {
    grammar homework_parser() for str {
        rule number() -> u64
            = (" " *) n:$(['0'..='9']+) (" " *) {? n.parse().or(Err("u64")) }

        rule add() -> Operation
            = "+" { Operation::Add }

        rule multiply() -> Operation
            = "*" { Operation::Multiply }

        rule operation() -> Operation
            = (" " *) o:(add() / multiply()) (" " *) { o }

        rule number_row() -> Vec<u64>
            = r:(number() +) "\n" { r }

        rule last_row() -> Vec<Operation>
            = r:(operation() +) ("\n" *) { r }

        pub rule homework() -> Vec<(Operation, Vec<u64>)>
            = nrs:(number_row() +) l:last_row() {
                let mut nrs = nrs;
                let mut ret = Vec::new();
                for op in l.into_iter().rev() {
                    let mut v = Vec::new();
                    for i in &mut nrs {
                        v.push(i.pop().unwrap());
                    }
                    ret.push((op, v))
                }
                ret
            }
    }
}

peg::parser! {
    grammar complex_homework_parser() for str {
        rule number() -> (usize, &'input str)
            = (" " *) p:position!() n:$(['0'..='9']+) (" " *) { (p, n) }
        
        rule add() -> Operation
            = "+" { Operation::Add }

        rule multiply() -> Operation
            = "*" { Operation::Multiply }

        rule operation() -> (usize, Operation)
            = (" " *) p:position!() o:(add() / multiply()) (" " *) { (p, o) }

        rule number_row() -> Vec<(usize, &'input str)>
            = p:position!() r:(number() +) "\n" {
                r.into_iter().map(|(x, i)| (x - p, i)).collect()
            }

        rule last_row() -> Vec<(usize, Operation)>
            = p:position!() r:(operation() +) ("\n" *) {
                r.into_iter().map(|(x, i)| (x - p, i)).collect()
            }

        pub rule homework() -> Vec<(Operation, Vec<u64>)>
            = nrs:(number_row() +) l:last_row() {
                let mut nrs = nrs;
                let mut ret = Vec::new();
                for (op_p, op) in l.into_iter().rev() {
                    let mut v = Vec::new();
                    for i in &mut nrs {
                        let (p, s) = i.pop().unwrap();
                        let rp = p - op_p;

                        for (dp, c) in (rp..).zip(s.chars()) {
                            while dp >= v.len() {
                                v.push(0);
                            }
                            v[dp] *= 10;
                            v[dp] += c.to_digit(10).unwrap() as u64;
                        }
                    }
                    ret.push((op, v))
                }
                ret
            }
    }
}
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

fn part1() -> String {
    let mut f = File::open("day6.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let homework = homework_parser::homework(&data).unwrap();

    let mut total = 0;

    for (op, v) in homework {
        total += v.into_iter().reduce(
            |acc, x| match op {
                Operation::Add => acc + x,
                Operation::Multiply => acc * x,
            }
        ).unwrap();
    }

    total.to_string()
}

fn part2() -> String {
    let mut f = File::open("day6.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let homework = complex_homework_parser::homework(&data).unwrap();

    let mut total = 0;
    
    for (op, v) in homework {
        total += v.into_iter().reduce(
            |acc, x| match op {
                Operation::Add => acc + x,
                Operation::Multiply => acc * x,
            }
        ).unwrap();
    }

    total.to_string()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());

    let mut c = Criterion::default();

    c.bench_function("day6_part1", |b| b.iter(|| part1()));

    c.bench_function("day6_part2", |b| b.iter(|| part2()));
}

