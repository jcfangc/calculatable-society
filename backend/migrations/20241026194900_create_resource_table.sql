-- Add migration script here

CREATE TABLE resource (
    id SERIAL PRIMARY KEY,
    numerator INT NOT NULL,
    denominator INT NOT NULL,
    allocatable BIGINT,
    investment BIGINT,
    debt BIGINT
);
