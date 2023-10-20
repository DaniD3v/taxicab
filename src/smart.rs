use crate::table_gen::generate_cube_table;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, PartialEq)]
struct Number {
    sum: u128,
    x: usize,
    y: usize
}

impl Number {
    pub fn new(x: usize, y: usize, table: &[u128]) -> Self {
        Self { sum: table[x] + table[y], x, y }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sum.cmp(&other.sum).reverse()
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// doesn't terminate at upper_bound!
pub fn search_taxicab(matches_needed: u8, upper_bound: u128) {
    let table = generate_cube_table(upper_bound);

    let mut queue = BinaryHeap::new();
    queue.push(Number::new(1, 1, &table));

    let mut cols: HashSet<usize> = HashSet::new(); // x
    let mut rows: HashSet<usize> = HashSet::new(); // y

    let mut previous = 0;
    let mut matches_count = 1;

    while let Some(Number {sum, x, y}) = queue.pop() {
        debug_assert!(sum >= previous);
        if sum % 100000000000 == 0 {println!("{:.2}% there!", (sum as f64/upper_bound as f64)*100.0);} // this is mathematically very incorrect

        if sum == previous {
            matches_count += 1;
            if matches_count >= matches_needed {println!("{sum} has {matches_count} matches");}
        } else {
            matches_count = 1;
            previous = sum;
        }

        cols.remove(&x);
        rows.remove(&y);

        let adjacent = x+1;
        if adjacent < table.len() && !cols.contains(&adjacent) {
            insert(&mut queue, &mut cols, &mut rows, &table, adjacent, y);
        }

        let below = y+1;
        if below <= x && below < table.len() && !rows.contains(&below) {
            insert(&mut queue, &mut cols, &mut rows, &table, x, below);
        }
    }
}

fn insert(queue: &mut BinaryHeap<Number>, cols: &mut HashSet<usize>, rows: &mut HashSet<usize>, table: &[u128], x: usize, y: usize) {
    queue.push(Number::new(x, y, table));

    cols.insert(x);
    rows.insert(y);
}

// TODO add explanation why this algorithm works
