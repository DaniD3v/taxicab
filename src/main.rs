mod naive;
mod smart;
mod table_gen;

fn main() {
    smart::search_taxicab(4, 6963472309248); // doable

    // smart::search_taxicab(5, 48988659276962496) // if you have 3TB of ram and 180 minutes ig
    // naive::bruteforce_taxicab(4, 87539319, 6963472309248); // way too slow
}