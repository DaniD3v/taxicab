
pub fn generate_cube_table(upper_bound: u128) -> Vec<u128> {
    let mut table = Vec::new();

    for i in 0u128.. {
        let result = i.pow(3);
        if result > upper_bound {break;}

        table.push(result);
    }

    table
}
