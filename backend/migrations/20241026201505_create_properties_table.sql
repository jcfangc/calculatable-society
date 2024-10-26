-- Add migration script here

-- migrations/YYYYMMDDHHMMSS_create_properties_table.sql

CREATE TABLE properties (
    resource_numerator INT NOT NULL,
    resource_dominator INT NOT NULL,
    flammable FLOAT NOT NULL,
    toxic FLOAT NOT NULL,
    reactive FLOAT NOT NULL,
    corrosive FLOAT NOT NULL,
    oxidizer FLOAT NOT NULL,
    acid_base FLOAT NOT NULL,
    phase FLOAT NOT NULL,
    conductive FLOAT NOT NULL,
    magnetic FLOAT NOT NULL,
    brittle FLOAT NOT NULL,
    malleable FLOAT NOT NULL,
    elastic FLOAT NOT NULL,
    transparent FLOAT NOT NULL,
    PRIMARY KEY (resource_numerator, resource_dominator)
);
