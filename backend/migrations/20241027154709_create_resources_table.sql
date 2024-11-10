-- Add migration script here

CREATE TABLE IF NOT EXISTS resources (
    record_id SERIAL PRIMARY KEY,
    agent_id UUID NOT NULL,
    subtance_numerator INTEGER NOT NULL CHECK (subtance_numerator > 0),
    subtance_denominator INTEGER NOT NULL CHECK (subtance_denominator > 0),
    allocatable INTEGER NOT NULL CHECK (allocatable >= 0),
    investment INTEGER NOT NULL CHECK (investment >= 0),
    debt INTEGER NOT NULL CHECK (debt >= 0),

    UNIQUE (agent_id, subtance_numerator, subtance_denominator)
);
