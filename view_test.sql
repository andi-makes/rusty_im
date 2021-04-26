CREATE VIEW overview AS
SELECT parts.id,
    parts.name,
    manufacturers.name as manufacturer,
    parts.amount,
    tagnames.name as type,
    tags.value
FROM (
        (
            (
                (
                    parts
                    LEFT JOIN manufacturers ON parts.manufacturer_id = manufacturers.id
                )
                LEFT JOIN part_tag ON parts.id = part_tag.part_id
            )
            LEFT JOIN tags ON part_tag.tag_id = tags.id
        )
        LEFT JOIN tagnames ON tags.tagname_id = tagnames.id
    );
SELECT *
FROM overview;