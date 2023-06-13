-- Add migration script here
CREATE TABLE books(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    author VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    pages INT NOT NULL,
    price NUMERIC(4, 2) NOT NULL,
    published_at timestamptz NOT NULL
);
