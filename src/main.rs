use std::collections::HashSet;

fn generate_cube_hashset(upper_bound: u128) -> HashSet<u128> {
    let mut table = HashSet::new();

    for i in 0u128.. {
        let result = i.pow(3);
        if result > upper_bound {break;}

        table.insert(result);
    }

    table
}

fn bruteforce_taxicab(needed_matches: u8, upper_bound: u128, lower_bound: u128) {
    let table = generate_cube_hashset(upper_bound);
    println!("Iterations needed per number: {}", table.len());

    let start = std::time::Instant::now();

    for i in lower_bound..lower_bound + 10000 {
        if i % 10000 == 0 {println!("Checking {i}");}
        let mut matches = 0;

        for tn in table.iter() {
            if tn > &i { continue; }
            if table.contains(&(i - tn)) {matches += 1;}
        }

        matches /= 2;
        if matches >= needed_matches-1 {
            println!("{i} has {matches} matches!")
        }
    }

    println!("{:#?} elapsed", start.elapsed());
}

fn main() {
    bruteforce_taxicab(4, 6963472309248, 87539319);
}
