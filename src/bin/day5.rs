use std::fs::File;
use std::io::Read;

peg::parser! {
    grammar list_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        rule range() -> (u64, u64)
            = a:number() "-" b:number() "\n" { (a, b) }
        
        rule id() -> u64
            = i:number() "\n" { i }

        pub rule ranges_and_ids() -> (Vec<(u64, u64)>, Vec<u64>)
            = r:(range() +) ("\n" *) i:(id() +) ("\n" *) { (r, i) }
    }
}

fn main() {
    let mut f = File::open("day5.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let (ranges, ids) = list_parser::ranges_and_ids(&data).unwrap();

    let mut total = 0;

    for id in ids {
        let found = ranges
            .iter()
            .find(|&&(a, b)| id >= a && id <= b)
            .is_some();

        if found {
            total += 1;
        }
    }

    println!("{total}");
}
