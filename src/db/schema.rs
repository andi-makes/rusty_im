table! {
    manufacturers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    part_tag (id) {
        id -> Int4,
        part_id -> Int4,
        tag_id -> Int4,
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

joinable!(part_tag -> parts (part_id));
joinable!(part_tag -> tags (tag_id));
joinable!(parts -> manufacturers (manufacturer_id));
joinable!(tags -> tagnames (tagname_id));

allow_tables_to_appear_in_same_query!(
    manufacturers,
    part_tag,
    parts,
    tagnames,
    tags,
);
