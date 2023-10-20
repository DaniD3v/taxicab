extern crate core;

mod table_gen;
mod naive;
mod smart;
mod dumb;

fn main() {
    // dumb::search_taxicab(3, 87539319);
    // dumb::search_taxicab(4, 6963472309248); // doable

    // smart::search_taxicab(2, 87539319);
    // smart::search_taxicab(4, 6963472309248);
    smart::search_taxicab(5, 48988659276962496)

    // naive::bruteforce_taxicab(4, 87539319, 6963472309248); // way too slow
}