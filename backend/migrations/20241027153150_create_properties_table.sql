-- Add migration script here

CREATE TABLE IF NOT EXISTS properties (
    resource_numerator INTEGER NOT NULL CHECK (resource_numerator > 0),
    resource_dominator INTEGER NOT NULL CHECK (resource_dominator > 0),
    flammable DOUBLE PRECISION NOT NULL,
    toxic DOUBLE PRECISION NOT NULL,
    reactive DOUBLE PRECISION NOT NULL,
    corrosive DOUBLE PRECISION NOT NULL,
    oxidizer DOUBLE PRECISION NOT NULL,
    acid_base DOUBLE PRECISION NOT NULL,
    phase DOUBLE PRECISION NOT NULL,
    conductive DOUBLE PRECISION NOT NULL,
    magnetic DOUBLE PRECISION NOT NULL,
    brittle DOUBLE PRECISION NOT NULL,
    malleable DOUBLE PRECISION NOT NULL,
    elastic DOUBLE PRECISION NOT NULL,
    transparent DOUBLE PRECISION NOT NULL,
    
    PRIMARY KEY (resource_numerator, resource_dominator)
);
