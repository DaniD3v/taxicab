use std::collections::HashSet;
use crate::other::generate_cube_table;

pub fn bruteforce_taxicab(matches_needed: u8, lower_bound: u128, upper_bound: u128) {
    let table_ordered = generate_cube_table(upper_bound);
    let table_set: HashSet<u128> = table_ordered.clone().into_iter().collect();

    println!("Iterations needed per number: {}", table_ordered.len());

    for i in lower_bound..=upper_bound {
        let mut matches = 0;

        for tn in table_ordered.iter() {
            if tn > &i { break; }
            if table_set.contains(&(i - tn)) {matches += 1;}
        }

        matches /= 2;
        if matches >= matches_needed { println!("{i}: {matches} matches") }
    }
}
