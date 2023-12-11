// @generated automatically by Diesel CLI.

diesel::table! {
    mtg_cards (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        released_at -> Varchar,
        #[max_length = 255]
        uri -> Varchar,
        #[max_length = 255]
        mana_cost -> Varchar,
        cmc -> Float4,
        #[max_length = 255]
        type_line -> Varchar,
        oracle_text -> Text,
        #[max_length = 255]
        loyalty -> Varchar,
        reserved -> Bool,
        promo -> Bool,
        reprint -> Bool,
        #[max_length = 255]
        set_id -> Varchar,
        #[max_length = 255]
        set_name -> Varchar,
        set_search_uri -> Text,
        rulings_uri -> Text,
        #[max_length = 255]
        collector_number -> Varchar,
        digital -> Bool,
        #[max_length = 255]
        rarity -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        #[max_length = 255]
        frame -> Varchar,
        #[max_length = 255]
        security_stamp -> Varchar,
        full_art -> Bool,
        textless -> Bool,
        edhrec_rank -> Int4,
        prices -> Jsonb,
        purchase_uris -> Jsonb,
    }
}
