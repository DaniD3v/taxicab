use std::collections::HashSet;
use crate::table_gen::generate_cube_table;

pub fn bruteforce_taxicab(matches_needed: u8, lower_bound: u128, upper_bound: u128) {
    let table_ordered = generate_cube_table(upper_bound);
    let table_set: HashSet<u128> = table_ordered.clone().into_iter().collect();

    println!("Iterations needed per number: {}", table_ordered.len());
    let mut start = std::time::Instant::now();

    for i in lower_bound..=upper_bound {
        let mut matches = 0;
        if i % 1000000 == 0 {
            println!("Checking {i} in {:#?}", start.elapsed());
            start = std::time::Instant::now();
        }

        for tn in table_ordered.iter() {
            if tn > &i { break; }
            if table_set.contains(&(i - tn)) {matches += 1;}
        }

        matches /= 2;
        if matches >= matches_needed -1 {
            println!("{i} has {matches} matches!")
        }
    }
}
