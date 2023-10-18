mod naive;
mod smart;
mod table_gen;

fn main() {
    smart::search_taxicab(4, 6963472309248);

    // naive::bruteforce_taxicab(4, 87539319, 6963472309248);
}