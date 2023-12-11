pub mod schema;

use diesel::{Insertable, Queryable, Selectable};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|err| {
        panic!("Error connecting to {}: {}", database_url, err);
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub usd_etched: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable, AsChangeset)]
#[diesel(table_name = schema::mtg_cards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Card {
    pub id: String,
    pub name: String,
    pub released_at: String,
    pub uri: String,
    #[serde(default)]
    pub mana_cost: String,
    #[serde(default)]
    pub cmc: f32,
    #[serde(default)]
    pub type_line: String,
    #[serde(default)]
    pub oracle_text: String,
    #[serde(default)]
    pub loyalty: String,
    pub reserved: bool,
    pub promo: bool,
    pub reprint: bool,
    pub set_id: String,
    pub set_name: String,
    pub set_search_uri: String,
    pub rulings_uri: String,
    pub collector_number: String,
    pub digital: bool,
    pub rarity: String,
    pub artist: String,
    pub frame: String,
    #[serde(default)]
    pub security_stamp: String,
    pub full_art: bool,
    pub textless: bool,
    #[serde(default)]
    pub edhrec_rank: i32,
    pub prices: diesel_json::Json<Prices>,
    pub purchase_uris: diesel_json::Json<Option<PurchaseUris>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUris {
    pub small: String,
    pub normal: String,
    pub large: String,
    pub png: String,
    pub art_crop: String,
    pub border_crop: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct AllParts {
    pub object: String,
    pub id: String,
    pub component: String,
    pub name: String,
    pub type_line: Option<String>,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legalities {
    pub standard: String,
    pub future: String,
    pub historic: String,
    pub gladiator: String,
    pub pioneer: String,
    pub explorer: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub oathbreaker: String,
    pub brawl: String,
    pub historicbrawl: String,
    pub alchemy: String,
    pub paupercommander: String,
    pub duel: String,
    pub oldschool: String,
    pub premodern: String,
    pub predh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedUris {
    pub tcgplayer_infinite_articles: Option<String>,
    pub tcgplayer_infinite_decks: Option<String>,
    pub edhrec: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseUris {
    pub tcgplayer: Option<String>,
    pub cardmarket: Option<String>,
    pub cardhoarder: Option<String>,
}

impl Prices {
    pub fn get_usd(&self) -> f32 {
        let usd = self.usd.clone().unwrap();
        let usd = usd.parse::<f32>().unwrap();
        usd
    }


    pub fn get_buylist(&self) -> f32 {
        let usd = self.usd.clone().unwrap_or_else(|| String::from("0.0"));
        // if usd.is_none() {
        //     return 0.0;
        // }
        let usd = usd.parse::<f32>().unwrap();
        let buylist = usd * 0.5;
        buylist
    }
}