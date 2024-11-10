-- Add migration script here

CREATE TABLE IF NOT EXISTS properties (
    record_id SERIAL PRIMARY KEY,
    subtance_numerator INTEGER NOT NULL CHECK (subtance_numerator > 0),
    subtance_denominator INTEGER NOT NULL CHECK (subtance_denominator > 0),
    frequency_constant INTEGER NOT NULL,
    phase_constant INTEGER NOT NULL,
    property_value FLOAT NOT NULL CHECK (property_value >= 0 AND property_value <= 1),

    UNIQUE (subtance_numerator, subtance_denominator, frequency_constant, phase_constant)
);
