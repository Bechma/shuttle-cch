use axum::Router;

mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;
mod day_11;
mod day_12;
mod day_13;
mod day_13_dummy;
mod day_14;
mod day_15;
mod day_18;

pub fn routes(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        .nest("/1", day_01::route())
        .nest("/4", day_04::route())
        .nest("/6", day_06::route())
        .nest("/7", day_07::route())
        .nest("/8", day_08::route())
        .nest("/11", day_11::route())
        .nest("/12", day_12::route())
        .nest("/13", day_13::route(pool.clone()))
        .nest("/13/dummy", day_13_dummy::route())
        .nest("/14", day_14::route())
        .nest("/15", day_15::route())
        .nest("/18", day_18::route(pool))
}

#[cfg(test)]
pub(crate) async fn routes_test() -> axum_test::TestServer {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = routes(pool).layer(tower_http::trace::TraceLayer::new_for_http());
    let config = axum_test::TestServerConfig::builder()
        // Preserve cookies across requests
        // for the session cookie to work.
        .save_cookies()
        .expect_success_by_default()
        .mock_transport()
        .build();

    axum_test::TestServer::new_with_config(app, config).unwrap()
}
