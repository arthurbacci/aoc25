use std::fs::File;
use std::io::Read;

use itertools::Itertools;

peg::parser! {
    grammar id_ranges_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        rule id_range() -> (u64, u64)
            = f:(number()) "-" l:(number()) { (f, l) }

        pub rule id_ranges() -> Vec<(u64, u64)>
            = l:(id_range() ** ",") ("\n" *) { l }
    }
}

struct InvalidIdGen<const N: u32> {
    part: u64,
}

impl<const N: u32> InvalidIdGen<N> {
    pub fn starting_with(start: u64) -> Self {
        let num_digit = start.max(1).ilog10() + 1;

        let first_part = if num_digit % N == 0 {
            start / 10_u64.pow(num_digit - num_digit / N)
        } else {
            10_u64.pow(num_digit / N)
        };

        let mut r = InvalidIdGen {
            part: first_part,
        };

        while r.peek() < start {
            r.next().unwrap();
        }

        r
    }

    pub fn peek(&self) -> u64 {
        let mut r = self.part;
        for i in 1..N {
            r += self.part * 10_u64.pow((self.part.ilog10() + 1) * i)
        }
        r
    }
}

impl<const N: u32> Iterator for InvalidIdGen<N> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.peek();
        self.part += 1;
        Some(r)
    }
}

pub fn all_starting_with(start: u64) -> impl Iterator<Item = u64> {
    let gens: Vec<Box<dyn Iterator<Item = u64>>> = vec![
        Box::new(InvalidIdGen::<2>::starting_with(start)),
        Box::new(InvalidIdGen::<3>::starting_with(start)),
        Box::new(InvalidIdGen::<5>::starting_with(start)),
        Box::new(InvalidIdGen::<7>::starting_with(start)),
        Box::new(InvalidIdGen::<13>::starting_with(start)),
        Box::new(InvalidIdGen::<17>::starting_with(start)),
    ];

    gens.into_iter().kmerge().dedup()
}


fn main() {
    let mut f = File::open("day2.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let id_ranges = id_ranges_parser::id_ranges(&data).unwrap();

    let mut total1 = 0;
    let mut total2 = 0;

    for (fst, lst) in id_ranges {
        total1 += InvalidIdGen::<2>::starting_with(fst)
            .take_while(|&x| x <= lst)
            .sum::<u64>();
        total2 += all_starting_with(fst)
            .take_while(|&x| x <= lst)
            .sum::<u64>();
    }

    println!("Part 1: {total1}");
    println!("Part 2: {total2}");
}
