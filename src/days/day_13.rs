use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

type Day13State = Arc<Mutex<HashMap<i64, Order>>>;

pub(super) fn route() -> Router {
    Router::new()
        .route("/sql", get(sql))
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/orders/total", get(total))
        .route("/orders/popular", get(popular))
        .with_state(Arc::new(Mutex::new(HashMap::<i64, Order>::new())))
}

#[derive(serde::Deserialize)]
struct Order {
    id: i64,
    // region_id: i64,
    gift_name: String,
    quantity: u64,
}

async fn sql() -> Json<i64> {
    Json(20231213)
}

async fn reset(State(db): State<Day13State>) {
    db.lock().unwrap().clear();
}

async fn insert_orders(State(db): State<Day13State>, Json(payload): Json<Vec<Order>>) {
    db.lock()
        .unwrap()
        .extend(payload.into_iter().map(|x| (x.id, x)));
}

#[derive(serde::Serialize)]
struct Total {
    total: u64,
}

async fn total(State(db): State<Day13State>) -> Json<Total> {
    Json(Total {
        total: db
            .lock()
            .unwrap()
            .values()
            .map(|order| order.quantity)
            .sum(),
    })
}

#[derive(serde::Serialize)]
struct Popular {
    popular: Option<String>,
}

async fn popular(State(db): State<Day13State>) -> Json<Popular> {
    let db = db.lock().unwrap();
    let mut counter = HashMap::<&str, u64>::new();
    for order in db.values() {
        counter
            .entry(&order.gift_name)
            .and_modify(|x| *x += order.quantity)
            .or_insert(order.quantity);
    }
    Json(Popular {
        popular: counter.iter().max_by_key(|x| x.1).map(|x| x.0.to_string()),
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[tokio::test]
    async fn all_tests() {
        let server = super::super::routes_test();
        // Task 1
        server.get("/13/sql").await.assert_json(&20231213i64);

        // Task 2
        server.post("/13/reset").await.assert_status_ok();
        server
            .post("/13/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":4,"region_id":4,"gift_name":"Board Game","quantity":10},
              {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Toy Train","quantity":3}
            ]))
            .await
            .assert_status_ok();
        server
            .get("/13/orders/total")
            .await
            .assert_json(&json!({"total": 44}));

        // Task 3
        server.post("/13/reset").await.assert_status_ok();
        server
            .get("/13/orders/popular")
            .await
            .assert_json(&json!({"popular": null}));
        server
            .post("/13/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Toy Train","quantity":4}
            ]))
            .await
            .assert_status_ok();

        server
            .get("/13/orders/popular")
            .await
            .assert_json(&json!({"popular": "Toy Train"}));
    }
}
