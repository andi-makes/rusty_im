CREATE TABLE manufacturers (
    id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR NOT NULL,
    PRIMARY KEY(id),
    UNIQUE(name)
);
CREATE TABLE parts (
    id INT GENERATED ALWAYS AS IDENTITY,
    manufacturer_id INT NOT NULL,
    name VARCHAR NOT NULL,
    amount INT NOT NULL DEFAULT 0,
    PRIMARY KEY(id),
    FOREIGN KEY(manufacturer_id) REFERENCES manufacturers(id) ON DELETE CASCADE,
    UNIQUE(name)
);
CREATE TABLE tagnames (
    id INT GENERATED ALWAYS AS IDENTITY,
    name VARCHAR NOT NULL,
    PRIMARY KEY(id),
    UNIQUE(name)
);
CREATE TABLE tags (
    id INT GENERATED ALWAYS AS IDENTITY,
    tagname_id INT NOT NULL,
    value VARCHAR NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(tagname_id) REFERENCES tagnames(id)
);
CREATE TABLE part_tag (
    id INT GENERATED ALWAYS AS IDENTITY,
    part_id INT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(part_id) REFERENCES parts(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);