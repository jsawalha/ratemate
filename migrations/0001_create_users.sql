CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY, -- Use BIGSERIAL to match Rust's i64
    name VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(250)
);
