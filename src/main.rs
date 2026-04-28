use axum::{Json, Router, extract::{Path, State}, http::StatusCode, routing::{get, post}};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool,}; // postgres operations
use tower_http::services::ServeDir;

use std::env;


// User struct to store users who are leaving review
#[derive(Serialize, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: Option<String>,
}

// Review struct to store reviews from movies 
// TODO: Add games, music, and books later
#[derive(Serialize, FromRow)]
struct Review {
    id: i64,
    user_id: i64,
    movie_title: String,
    rating: f64, 
    genre: Option<String>,
    notes: Option<String>,
    created_at: Option<chrono::NaiveDate>
}


#[derive(Deserialize)] // This is what the client will send to you, you need to deserialize it because you're reading FROM a JSON, not writing
struct CreateReview {
    user_id: i64,
    movie_title: String,
    rating: f64, 
    genre: Option<String>,
    notes: Option<String>,
}

#[derive(Serialize, FromRow)]
struct LeaderboardEntry {
    name: String,
    avg_rating: f64,
}

#[derive(Serialize, FromRow)]
struct ReviewWithUser {
    id: i64,
    user_id: i64,
    movie_title: String,
    rating: f64,
    genre: Option<String>,
    notes: Option<String>,
    name: String,
    created_at: Option<chrono::NaiveDate>
}

#[tokio::main]
async fn main() {
    // db_url, this will connect our backend will connect the backend with the database.
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set properly...");
    // let pool be a way to structure and a trial to connect to the database
    let pool = PgPoolOptions::new().connect(&db_url).await.expect("Failed to connect to DB");
    // this command will go into migrations 00001_create_users.sql and run the sql command! MAKES THE TABLE
    // migrate! is actually a macro that automatically connects to migrations folder
    sqlx::migrate!().run(&pool).await.expect("Could not migrate and create SQL table");

    // get router for now
    let app = Router::new()
        .fallback_service(ServeDir::new("static"))
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/reviews", post(create_review).get(get_reviews))
        .route("/reviews/full", get(get_review_full))
        .route("/reviews/{id}", get(get_review))
        .route("/leaderboard", get(get_leaderboard))
        .with_state(pool);

    // CREATE THE LISTENER: This "claims" the port from the Operating System.
    //    - "0.0.0.0" means "listen to any incoming connection from the network."
    //    - .await is needed because opening a network socket takes a moment.
    //    - .unwrap() crashes the app if the port is already taken (e.g., by another app).
    let port_number = 8000;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Server is running on port: {port_number}");
    // START THE ENGINE: This hands the "phone line" (listener) to the "operator" (axum).
    //    - 'app' is your Router that knows which URLs go to which functions.
    //    - .await stays here forever, keeping the program alive to handle requests.
    axum::serve(listener, app).await.unwrap();


}


// Endpoint handlers


// get_user endpoint
async fn list_users(state: State<PgPool>) -> Result<Json<Vec<User>>, StatusCode> { // Axum injects the shared pool from .with_state(pool) in main
    sqlx::query_as("SELECT * FROM users")  // like query! but maps results directly into a struct.
    .fetch_all(&state.0) // runs the query and collects every row into a Vec<User>. &state.0 unwraps the State wrapper to get the &PgPool reference SQLx needs
    .await
    .map(Json) // if the result is Ok(vec), wraps it in Json(vec) so Axum serializes it as a JSON response. Json here is used as a function (it's a tuple struct constructor)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// create_user endpoint
// we need a POST request for this which means we need a Json payload 

async fn create_user(
    State(pool): State<PgPool>,
    Json(create_user): Json<CreateUser> ) -> Result<(StatusCode, Json<User>), StatusCode> {
        sqlx::query_as("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *") // why returning don't know
        .bind(create_user.name)
        .bind(create_user.email)
        .fetch_one(&pool).await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(|e| {
            eprintln!("create_user error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn get_user(
    state: State<PgPool>,
    Path(id): Path<i64>) -> Result<Json<User>, StatusCode> {
    sqlx::query_as("SELECT * from users WHERE id = $1")
    .bind(id)
    .fetch_one(&state.0).await
    .map(Json) // why just json here, why not json user
    .map_err(|_| StatusCode::NOT_FOUND)

}


//UPDATE USER
async fn update_user(
    state: State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateUser>
    ) -> Result<Json<User>, StatusCode> {
    sqlx::query_as::<_, User>("UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *")
        .bind(payload.name)
        .bind(payload.email)
        .bind(id)
        .fetch_one(&state.0).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

//DELETE USER
async fn delete_user(
    state: State<PgPool>,
    Path(id): Path<i64>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx
        ::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&state.0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// Post reviews
async fn create_review(
    State(pool): State<PgPool>,
    Json(create_review): Json<CreateReview> ) -> Result<(StatusCode, Json<Review>), StatusCode> {
        sqlx::query_as("INSERT INTO reviews (user_id, movie_title, rating, genre, notes) VALUES ($1, $2, $3, $4, $5) RETURNING id, user_id, movie_title, rating::FLOAT8 AS rating, genre, notes, created_at
") // why returning don't know
        .bind(create_review.user_id)
        .bind(create_review.movie_title)
        .bind(create_review.rating)
        .bind(create_review.genre)
        .bind(create_review.notes)
        .fetch_one(&pool).await
        .map(|u| (StatusCode::CREATED, Json(u)))
        .map_err(|e| {
            eprintln!("create_review error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}


async fn get_reviews(state: State<PgPool>) -> Result<Json<Vec<Review>>, StatusCode> { // Axum injects the shared pool from .with_state(pool) in main
    sqlx::query_as("SELECT id, user_id, movie_title, rating::FLOAT8 AS rating, genre, notes, created_at FROM reviews
")  // like query! but maps results directly into a struct.
    .fetch_all(&state.0) // runs the query and collects every row into a Vec<User>. &state.0 unwraps the State wrapper to get the &PgPool reference SQLx needs
    .await
    .map(Json) // if the result is Ok(vec), wraps it in Json(vec) so Axum serializes it as a JSON response. Json here is used as a function (it's a tuple struct constructor)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}


async fn get_review(
    state: State<PgPool>,
    Path(id): Path<i64>) -> Result<Json<Review>, StatusCode> {
    sqlx::query_as("SELECT id, user_id, movie_title, rating::FLOAT8 AS rating, genre, notes, created_at FROM reviews WHERE id = $1
")
    .bind(id)
    .fetch_one(&state.0).await
    .map(Json) // why just json here, why not json user
    .map_err(|_| StatusCode::NOT_FOUND)

}


async fn get_leaderboard(State(pool): State<PgPool>) -> Result<Json<Vec<LeaderboardEntry>>, StatusCode> { // Axum injects the shared pool from .with_state(pool) in main
    sqlx::query_as("SELECT users.name, AVG(reviews.rating)::FLOAT8 AS avg_rating FROM reviews JOIN users on users.id = reviews.user_id  GROUP BY users.name ")  // like query! but maps results directly into a struct.
    .fetch_all(&pool) // runs the query and collects every row into a Vec<User>. &state.0 unwraps the State wrapper to get the &PgPool reference SQLx needs
    .await
    .map(Json) // if the result is Ok(vec), wraps it in Json(vec) so Axum serializes it as a JSON response. Json here is used as a function (it's a tuple struct constructor)
    .map_err(|e| {
    eprintln!("leaderboard error: {e}");
    StatusCode::INTERNAL_SERVER_ERROR
})
}

async fn get_review_full(State(pool): State<PgPool>) -> Result<Json<Vec<ReviewWithUser>>, StatusCode> {
    sqlx::query_as("SELECT reviews.id, reviews.user_id, reviews.movie_title, reviews.rating::FLOAT8 AS rating, reviews.genre, reviews.notes, reviews.created_at, users.name FROM reviews JOIN users ON reviews.user_id = users.id ORDER BY reviews.id DESC
")  // like query! but maps results directly into a struct.
    .fetch_all(&pool) // runs the query and collects every row into a Vec<User>. &state.0 unwraps the State wrapper to get the &PgPool reference SQLx needs
    .await
    .map(Json) // if the result is Ok(vec), wraps it in Json(vec) so Axum serializes it as a JSON response. Json here is used as a function (it's a tuple struct constructor)
            .map_err(|e| {
            eprintln!("create_user error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
