use diesel::prelude::*;
use new_mtg::*;

fn main() {
    use self::schema::mtg_cards::dsl::*;

    let connection = &mut establish_connection();
    let results = mtg_cards
        .filter(name.eq("Abundance"))
        .limit(5)
        .load::<Card>(connection)
        .expect("Error loading cards");

    println!("Displaying {} cards", results.len());

}