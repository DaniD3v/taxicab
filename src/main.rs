extern crate core;

mod other;
mod naive;
mod smart;
mod dumb;

fn main() {
    println!("naive:"); // TODO add time + space complexity here
    naive::bruteforce_taxicab(2, 2, 1729);
    println!();

    println!("dumb:");
    dumb::search_taxicab(3, 87539319);
    // dumb::search_taxicab(4, 6963472309248); // barely doable (memory)
    println!();

    println!("smart:");
    smart::search_taxicab(4, 6963472309248);
    // smart::search_taxicab(5, 48988659276962496)
    println!();

    other::get_matches(6963472309248);
}