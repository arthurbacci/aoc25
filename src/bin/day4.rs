use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};

peg::parser! {
    grammar rolls_parser() for str {
        rule empty_cell() -> Cell
            = "." { Cell::Empty }

        rule roll_cell() -> Cell
            = "@" { Cell::Roll }

        rule cell() -> Cell
            = empty_cell() / roll_cell()

        rule row() -> Vec<Cell>
            = r:(cell() +) ("\n" ?) {
                [
                    vec![Cell::Empty],
                    r,
                    vec![Cell::Empty],
                ].concat()
            }

        pub rule grid() -> CellGrid
            = g:(row() +) ("\n" *) {
                let l = g[0].len();
                [
                    vec![vec![Cell::Empty; l]],
                    g,
                    vec![vec![Cell::Empty; l]],
                ].concat().into()
            }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Roll,
}

pub struct CellGrid(Vec<Vec<Cell>>);

impl CellGrid {
    pub fn width(&self) -> usize {
        self.0[0].len() - 2
    }
    pub fn height(&self) -> usize {
        self.0.len() - 2
    }

    pub fn neighborhoods(&self) -> Vec<((usize, usize), usize)> {
        let mut r = Vec::new();

        for y in 1..=self.height() {
            for x in 1..=self.width() {
                let n = [
                    self.0[y - 1][x - 1], self.0[y - 1][x], self.0[y - 1][x + 1],
                    self.0[y][x - 1], self.0[y][x + 1],
                    self.0[y + 1][x - 1], self.0[y + 1][x], self.0[y + 1][x + 1],
                ].into_iter()
                    .filter(|x| *x == Cell::Roll)
                    .count();

                r.push(((x, y), n));
            }
        }

        r
    }
}

impl From<Vec<Vec<Cell>>> for CellGrid {
    fn from(v: Vec<Vec<Cell>>) -> Self {
        CellGrid(v)
    }
}

impl Index<(usize, usize)> for CellGrid {
    type Output = Cell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}
impl IndexMut<(usize, usize)> for CellGrid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

fn main() {
    let mut f = File::open("day4.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    let mut grid = rolls_parser::grid(&data).unwrap();

    let mut part1 = 0u32;
    let mut part2 = 0u32;

    for removal in 1.. {
        let mut increment = 0;

        for (i, n) in grid.neighborhoods() {
            if grid[i] == Cell::Roll && n < 4 {
                increment += 1;
                grid[i] = Cell::Empty;
            }
        }

        if increment == 0 {
            break;
        }

        if removal == 1 {
            part1 += increment;
        }
        part2 += increment;
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
