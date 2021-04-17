table! {
    manufacturers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    parts (id) {
        id -> Int4,
        manufacturer_id -> Int4,
        name -> Varchar,
        amount -> Int4,
    }
}

joinable!(parts -> manufacturers (manufacturer_id));

allow_tables_to_appear_in_same_query!(
    manufacturers,
    parts,
);
