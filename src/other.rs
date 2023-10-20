use std::collections::HashSet;

pub fn generate_cube_table(upper_bound: u128) -> Vec<u128> {
    let mut table = Vec::new();

    for i in 0u128.. {
        let result = i.pow(3);
        if result > upper_bound {break;}

        table.push(result);
    }

    table
}

pub fn get_matches(number: u128) {
    println!("{number} can be represented with the following sums of cubes:");

    let table_ordered = generate_cube_table(number);
    let mut table_set: HashSet<u128> = table_ordered.clone().into_iter().collect();

    for (tni, tn) in table_ordered.iter().enumerate() {
        if tn > &number { break; } // works because the table is ordered by size

        if table_set.contains(&(number - tn)) {
            println!("{tni}³ + {:.0}³ = {number}", ((number - tn) as f64).powf(1.0 / 3.0));
            table_set.remove(&tn);
        }
    }
}
