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
    PRIMARY KEY(id),
    FOREIGN KEY(manufacturer_id) REFERENCES manufacturers(id) ON DELETE CASCADE,
    UNIQUE(name)
);