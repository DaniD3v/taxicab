use std::collections::HashSet;

fn generate_cube_table(upper_bound: u128) -> Vec<u128> {
    let mut table = Vec::new();

    for i in 0u128.. {
        let result = i.pow(3);
        if result > upper_bound {break;}

        table.push(result);
    }

    table
}

fn bruteforce_taxicab(needed_matches: u8, lower_bound: u128, upper_bound: u128) {
    let table_ordered = generate_cube_table(upper_bound);
    let table_set: HashSet<u128> = table_ordered.clone().into_iter().collect();

    println!("Iterations needed per number: {}", table_ordered.len());
    let mut start = std::time::Instant::now();

    for i in lower_bound..upper_bound {
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
        if matches >= needed_matches-1 {
            println!("{i} has {matches} matches!")
        }
    }
}

fn main() {
    bruteforce_taxicab(4, 87539319, 6963472309248);
}
