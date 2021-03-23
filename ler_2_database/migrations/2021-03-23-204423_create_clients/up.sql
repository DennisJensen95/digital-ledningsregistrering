CREATE TABLE clients
(
    id SERIAL PRIMARY KEY,
    user_name VARCHAR NOT NULL,
    data_file TEXT NOT NULL
)