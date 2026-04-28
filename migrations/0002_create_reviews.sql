CREATE TABLE reviews (
    id BIGSERIAL PRIMARY KEY, -- This creates the sequence automatically
    user_id BIGINT NOT NULL REFERENCES users(id),
    movie_title VARCHAR(100) NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 0 AND rating <= 10),
    genre VARCHAR(50),
    notes TEXT,
    created_at DATE DEFAULT CURRENT_DATE
);
