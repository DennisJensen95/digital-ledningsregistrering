CREATE TABLE clients
(
    id SERIAL PRIMARY KEY,
    company VARCHAR NOT NULL,
    email TEXT NOT NULL UNIQUE,
    data_file TEXT NOT NULL DEFAULT 'unknown',
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)