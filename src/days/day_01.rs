use std::ops::BitXor;

use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/*numbers", get(numbers))
}

async fn numbers(Path(numbers): Path<String>) -> Json<i64> {
    Json(
        numbers
            .trim_end_matches('/')
            .split('/')
            .map(|x| x.parse::<i64>().unwrap())
            .fold(0, |a, acc| a.bitxor(acc))
            .pow(3),
    )
}
