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

table! {
    tagnames (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tagname_id -> Int4,
        value -> Varchar,
    }
}

joinable!(parts -> manufacturers (manufacturer_id));
joinable!(tags -> tagnames (tagname_id));

allow_tables_to_appear_in_same_query!(
    manufacturers,
    parts,
    tagnames,
    tags,
);
