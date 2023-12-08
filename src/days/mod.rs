use axum::Router;

mod day_01;
mod day_04;
mod day_06;
mod day_07;
mod day_08;

pub fn routes() -> Router {
    Router::new()
        .nest("/1", day_01::route())
        .nest("/4", day_04::route())
        .nest("/6", day_06::route())
        .nest("/7", day_07::route())
        .nest("/8", day_08::route())
}
