-- Add migration script here

CREATE TABLE IF NOT EXISTS preferences (
    record_id SERIAL PRIMARY KEY,
    agent_id UUID NOT NULL,
    numerator INTEGER NOT NULL CHECK (numerator > 0),
    denominator INTEGER NOT NULL CHECK (denominator > 0),
    preference DOUBLE PRECISION NOT NULL CHECK (preference >= 0 AND preference <= 1),
    
    UNIQUE (agent_id, numerator, denominator)
);
