-- Add migration script here

CREATE TABLE IF NOT EXISTS preferences (
    record_id SERIAL PRIMARY KEY,
    agent_id UUID NOT NULL,
    subtance_numerator INTEGER NOT NULL CHECK (subtance_numerator > 0),
    subtance_denominator INTEGER NOT NULL CHECK (subtance_denominator > 0),
    preference DOUBLE PRECISION NOT NULL CHECK (preference >= 0 AND preference <= 1),
    
    UNIQUE (agent_id, subtance_numerator, subtance_denominator)
);
