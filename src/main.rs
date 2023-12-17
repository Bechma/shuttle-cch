use axum::http::StatusCode;
use axum::{routing::get, Router};

mod days;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_err() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let local_path = std::env::var("DATABASE_URL").unwrap_or(String::from("sqlite::memory:"));
    let pool = sqlx::SqlitePool::connect(&local_path).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_err))
        .nest("/", days::routes(pool))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    Ok(router.into())
}
