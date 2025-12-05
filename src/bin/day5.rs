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


fn search_id(id: u64, ranges: &[(u64, u64)]) -> Option<(u64, u64)> {
    let in_this_one = |i: usize| ranges[i].0 <= id && ranges[i].1 >= id;
    match ranges.len() {
        0 => None,
        1 => (in_this_one(0)).then_some(ranges[0]),
        n => {
            (in_this_one(n / 2).then_some(ranges[n / 2]))
                .or(
                    if ranges[n / 2].0 > id {
                        search_id(id, &ranges[0..(n / 2)])
                    } else if n / 2 + 1 < ranges.len() {
                        search_id(id, &ranges[(n / 2 + 1)..])
                    } else {
                        None
                    }
                )
        }
    }
}


fn unintersect(ranges: &mut Vec<(u64, u64)>) {
    let mut i = 0;
    loop {
        if i + 1 >= ranges.len() {
            break;
        }
        if ranges[i + 1].0 <= ranges[i].1 {
            ranges[i].1 = ranges[i].1.max(ranges[i + 1].1);
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut f = File::open("day5.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let (mut ranges, ids) = list_parser::ranges_and_ids(&data).unwrap();

    ranges.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    println!("{}", ranges.len());

    unintersect(&mut ranges);

    println!("{}", ranges.len());

    let mut part1 = 0;
    let mut part2 = 0;
    
    for id in ids {
        if search_id(id, &ranges).is_some() {
            part1 += 1;
        }
    }

    for range in ranges {
        part2 += range.1 + 1 - range.0;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
