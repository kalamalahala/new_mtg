use new_mtg::Card;
use std::fs::read_to_string;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    // read ./resources/card-snip.json to string
    let card_json = read_to_string("src/resources/card-snip.json").unwrap_or_else(|err| {
        panic!("Could not read file: {}", err);
    });

    // deserialize the string into a Card struct
    let card: Vec<Card> = serde_json::from_str(&card_json).unwrap();

    // let name = card[84862].name.clone();
    // let mut text = card[84862].oracle_text.clone();
    // if text.is_none() {
    //     text = Option::from(String::from("-"));
    // }
    // let mut usd_price = card[84862].prices.usd.clone();
    // if usd_price.is_none() {
    //     usd_price = Option::from(String::from("0.0"));
    // }

    println!("Looking through {} cards...", card.len());
    println!("{:?}", card);
    // println!("{}\n==============", name);
    // println!("{}", text.unwrap());
    // println!("USD: ${}", usd_price.unwrap());
    // println!("Buylist: ${:.2}", card[84862].prices.get_buylist());

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
}