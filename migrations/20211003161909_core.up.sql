-- Core up migration script

CREATE TABLE batch (
    id SERIAL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    id_batch INTEGER,
    deleted DATE,
    PRIMARY KEY(id),
    FOREIGN KEY(id_batch) REFERENCES batch(id)
    ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS item (
    id SERIAL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    id_batch INTEGER NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(id_batch) REFERENCES batch(id)
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS model (
    id SERIAL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    validation TEXT NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS property (
    id SERIAL,
    value TEXT NOT NULL,
    id_item INTEGER NOT NULL,
    id_model INTEGER NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(id_item) REFERENCES item(id)
    ON DELETE CASCADE,
    FOREIGN KEY(id_model) REFERENCES model(id)
    ON DELETE CASCADE
);