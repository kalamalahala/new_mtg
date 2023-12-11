use std::fs::read_to_string;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::pg::upsert::*;
use new_mtg::*;

fn main() {
    use schema::mtg_cards::dsl::*;

    let connection = &mut establish_connection();

    // read ./resources/card-snip.json to string
    let card_json = read_to_string("src/resources/cards.json").unwrap_or_else(|err| {
        panic!("Could not read file: {}", err);
    });

    let card: Vec<Card> = serde_json::from_str(&card_json).unwrap();

    println!("Beginning insert ...");

    for i in 0..card.len() {
        println!("Insert: {}", card[i].name);
        let count = insert_into(mtg_cards)
            .values(&card[i])
            .on_conflict(id)
            .do_update()
            .set(&card[i])
            .execute(connection)
            .expect("Error inserting cards");
    }

    // insert_into(mtg_cards)
    //     .values(&card)
    //     .execute(connection)
    //     .expect("Error inserting cards");
}

