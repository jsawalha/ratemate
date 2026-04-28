# RateMate 🎬

A movie review API built with Rust — one of my first full-stack projects. Me and my friends used to share movie reviews in a WhatsApp group chat, so I decided to build a dedicated tool for it instead. This was my first time building a real web service in Rust using Axum and SQLx, and my first time working with Docker and PostgreSQL in a production-like setup.

## What it does

RateMate lets 4 users log movie reviews (title, rating out of 10, genre, notes) and see each other's ratings. It includes a leaderboard showing each user's average rating, and a filterable reviews table so you can browse by reviewer, genre, or movie title.

## Tech Stack

- **Rust** — backend language
- **Axum** — web framework
- **SQLx** — async database queries with compile-time SQL checking
- **PostgreSQL** — database
- **Docker Compose** — orchestrates the app and database containers
- **HTML/CSS/JavaScript** — simple frontend served directly by Axum

## Running Locally

You need Docker installed. Then:

```bash
git clone https://github.com/YOUR_USERNAME/ratemate.git
cd ratemate

docker compose up --build
```

Open `http://localhost:8000` in your browser.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Health check |
| GET | `/users` | List all users |
| POST | `/users` | Create a user |
| GET | `/users/:id` | Get a single user |
| PUT | `/users/:id` | Update a user |
| DELETE | `/users/:id` | Delete a user |
| GET | `/reviews` | List all reviews |
| POST | `/reviews` | Submit a review |
| GET | `/reviews/:id` | Get a single review |
| GET | `/reviews/full` | All reviews with reviewer names (JOIN) |
| GET | `/leaderboard` | Average rating per user |

## Project Structure

```
ratemate/
├── Cargo.toml
├── compose.yml
├── Dockerfile
├── migrations/
│   ├── 0001_create_users.sql
│   └── 0002_create_reviews.sql
├── static/
│   └── index.html
└── src/
    └── main.rs
```

## What I Learned

This was one of my first real full-stack projects and my first time building a web service in Rust. Coming in with a beginner Rust background (Rustlings, Exercism), I had to figure out a lot of new concepts along the way:

- How Axum's extractor system works — `State<T>`, `Json<T>`, `Path<T>`
- How SQLx maps database rows to Rust structs with `FromRow`
- Why Rust types need to match database column types exactly
- How Docker Compose wires together multiple services
- How database migrations work and why they're better than writing schema in code
- The difference between `Serialize` and `Deserialize` and when you need each one

The hardest parts were getting Docker to compile Rust without crashing (ended up building locally and copying the binary in), and understanding SQLx's type system when mapping Postgres types to Rust types.
