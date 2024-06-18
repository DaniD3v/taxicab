use std::collections::HashMap;
use crate::other::generate_cube_table;

fn generate_table_pairs(upper_bound: u128) -> HashMap<u128, u8> {
    let table = generate_cube_table(upper_bound);

    let pairs = table.len()*(table.len()+1) / 2; // triangle number (factorial with addition)
    println!("Calculating {} columns, {} pairs", table.len(), pairs);

    let mut sum_table: HashMap<u128, u8> = HashMap::with_capacity(pairs);

    for (tni, tn) in table.iter().enumerate() {
        for otn in table.iter().skip(tni) {
            let entry = sum_table.entry(tn + otn).or_insert(0);
            *entry += 1;
        }
    }

    sum_table
}

pub fn search_taxicab(matches_needed: u8, upper_bound: u128) {
    let sum_table = generate_table_pairs(upper_bound);

    for (number, matches) in sum_table {
        if matches >= matches_needed { println!("{number}: {matches} matches"); }
    }
}

/*
    table:
    1 2 3 4
    1 2 3 4

    sums:
    1 + 1
    1 + 2 // 1. duplicate
    1 + 3 // 2. duplicate
    1 + 4 // 4. duplicate
    2 + 1 // 1. duplicate
    2 + 2
    2 + 3 // 3. duplicate
    2 + 4 // 5. duplicate
    3 + 1 // 2. duplicate
    3 + 2 // 3. duplicate
    3 + 3
    3 + 4 // 6. duplicate
    4 + 1 // 4. duplicate
    4 + 2 // 5. duplicate
    4 + 3 // 6. duplicate
    4 + 4

    sums optimized:
    1 + 1
    1 + 2
    1 + 3
    1 + 4
    2 + 2
    2 + 3
    2 + 4
    3 + 3
    3 + 4
    4 + 4

    calculate amount of items needed with the nth triangle number: https://math.stackexchange.com/questions/593318/factorial-but-with-addition
*/
