-- Add migration script here

CREATE TABLE IF NOT EXISTS property (
    record_id SERIAL PRIMARY KEY,
    resource_numerator INTEGER NOT NULL CHECK (resource_numerator > 0),
    resource_dominator INTEGER NOT NULL CHECK (resource_dominator > 0),
    frequency_constant INTEGER NOT NULL,
    phase_constant INTEGER NOT NULL,
    property_value FLOAT NOT NULL CHECK (property_value >= 0 AND property_value <= 1),

    UNIQUE (resource_numerator, resource_dominator, frequency_constant, phase_constant)
);
