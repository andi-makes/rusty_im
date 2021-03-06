CREATE TABLE manufacturers (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE(name)
);
CREATE TABLE parts (
    id INTEGER NOT NULL,
    manufacturer_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    amount INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY(id),
    FOREIGN KEY(manufacturer_id) REFERENCES manufacturers(id) ON DELETE CASCADE,
    UNIQUE(name, manufacturer_id)
);
CREATE TABLE tagnames (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    PRIMARY KEY(id),
    UNIQUE(name)
);
CREATE TABLE tags (
    id INTEGER NOT NULL,
    tagname_id INTEGER NOT NULL,
    value TEXT NOT NULL,
    UNIQUE(tagname_id, value),
    PRIMARY KEY(id),
    FOREIGN KEY(tagname_id) REFERENCES tagnames(id)
);
CREATE TABLE part_tag (
    id INTEGER NOT NULL,
    part_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    UNIQUE(part_id, tag_id),
    PRIMARY KEY(id),
    FOREIGN KEY(part_id) REFERENCES parts(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);