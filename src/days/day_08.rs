use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use rustemon::client::RustemonClient;

pub(super) fn route() -> Router {
    let rustemon_client = Arc::new(RustemonClient::default());
    Router::new()
        .route("/weight/:pokemon", get(weight))
        .route("/drop/:pokemon", get(drop))
        .with_state(rustemon_client)
}

async fn weight(Path(pokemon): Path<i64>, State(client): State<Arc<RustemonClient>>) -> Json<i64> {
    rustemon::pokemon::pokemon::get_by_id(pokemon, &client)
        .await
        .map(|x| Json(x.weight / 10))
        .unwrap()
}

const G: f64 = 9.825;

async fn drop(Path(pokemon): Path<i64>, State(client): State<Arc<RustemonClient>>) -> Json<f64> {
    rustemon::pokemon::pokemon::get_by_id(pokemon, &client)
        .await
        .map(|x| {
            let mass = (x.weight / 10) as f64;
            let kinetic_energy = mass * G * 10f64;
            let velocity = (2f64 * kinetic_energy / mass).sqrt();
            Json(velocity * mass)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test().get("/8/weight/25").await.assert_json(&6);
    }

    #[tokio::test]
    async fn task2() {
        let res: f64 = routes_test().get("/8/drop/25").await.json();
        assert!(res - 84.10707 <= 0.001 || res - 84.10707 > 0.001)
    }
}
