use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

pub(super) fn route(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        .route("/sql", get(sql))
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/orders/total", get(total))
        .route("/orders/popular", get(popular))
        .with_state(pool)
}

#[derive(serde::Deserialize)]
struct Order {
    id: i64,
    region_id: i64,
    gift_name: String,
    quantity: i64,
}

async fn sql(State(db): State<sqlx::SqlitePool>) -> Json<i32> {
    let res: i32 = sqlx::query_scalar("SELECT 20231213")
        .fetch_one(&db)
        .await
        .unwrap();
    Json(res)
}

async fn reset(State(db): State<sqlx::SqlitePool>) {
    // No TRUNCATE TABLE in sqlite
    sqlx::query("DELETE FROM orders")
        .execute(&db)
        .await
        .unwrap();
}

async fn insert_orders(State(db): State<sqlx::SqlitePool>, Json(payload): Json<Vec<Order>>) {
    let mut tx = db.begin().await.unwrap();
    for order in &payload {
        sqlx::query(
            r"INSERT INTO orders (id, region_id, gift_name, quantity)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(order.id)
        .bind(order.region_id)
        .bind(&order.gift_name)
        .bind(order.quantity)
        .execute(tx.as_mut())
        .await
        .unwrap();
    }
    tx.commit().await.unwrap();
}

#[derive(serde::Serialize)]
struct Total {
    total: i64,
}

async fn total(State(db): State<sqlx::SqlitePool>) -> Json<Total> {
    Json(Total {
        total: sqlx::query_scalar("SELECT SUM(quantity) FROM orders")
            .fetch_one(&db)
            .await
            .unwrap(),
    })
}

#[derive(serde::Serialize)]
struct Popular {
    popular: Option<String>,
}

async fn popular(State(db): State<sqlx::SqlitePool>) -> Json<Popular> {
    Json(Popular {
        popular: sqlx::query_scalar(
            "SELECT gift_name FROM orders GROUP BY gift_name ORDER BY SUM(quantity) DESC LIMIT 1",
        )
        .fetch_optional(&db)
        .await
        .unwrap(),
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[tokio::test]
    async fn all_tests() {
        let server = super::super::routes_test().await;
        // Task 1
        server.get("/13/sql").await.assert_json(&20_231_213_i64);

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
