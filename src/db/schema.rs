table! {
    manufacturers (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    part_tag (id) {
        id -> Integer,
        part_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    parts (id) {
        id -> Integer,
        manufacturer_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        amount -> Integer,
    }
}

table! {
    tagnames (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        tagname_id -> Integer,
        value -> Text,
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
