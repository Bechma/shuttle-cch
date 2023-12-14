use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Datelike, TimeZone};
use tokio::time::Instant;

type Day12State = Arc<Mutex<HashMap<String, Instant>>>;

pub(super) fn route() -> Router {
    Router::new()
        .route("/ulids", post(task2))
        .route("/ulids/:weekday", post(task3))
        .route("/save/:elem", post(save))
        .route("/load/:elem", get(load))
        .with_state(Arc::new(Mutex::new(HashMap::<String, Instant>::new())))
}

async fn save(Path(elem): Path<String>, State(state): State<Day12State>) -> StatusCode {
    state.lock().unwrap().insert(elem, Instant::now());
    StatusCode::OK
}

async fn load(Path(elem): Path<String>, State(state): State<Day12State>) -> Json<u64> {
    state
        .lock()
        .unwrap()
        .get(&elem)
        .map_or(Json(0), |x| Json(x.elapsed().as_secs()))
}

async fn task2(Json(payload): Json<Vec<String>>) -> Json<Vec<String>> {
    Json(
        payload
            .into_iter()
            .rev()
            .map(|x| uuid::Uuid::from_u128(ulid::Ulid::from_string(&x).unwrap().0).to_string())
            .collect(),
    )
}

#[derive(serde::Serialize, Default)]
struct Task3Output {
    #[serde(rename = "christmas eve")]
    christmas_eve: u64,
    weekday: u64,
    #[serde(rename = "in the future")]
    in_the_future: u64,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: u64,
}

async fn task3(Path(weekday): Path<u8>, Json(payload): Json<Vec<String>>) -> Json<Task3Output> {
    let mut output = Task3Output::default();
    let now = chrono::Utc::now();
    let weekday = chrono::Weekday::try_from(weekday).unwrap();

    for s in payload {
        let input: ulid::Ulid = s.parse().unwrap();
        let timestamp_ms = input.timestamp_ms() as i64;
        let datetime = chrono::Utc.timestamp_millis_opt(timestamp_ms).unwrap();
        if datetime.month() == 12 && datetime.day() == 24 {
            output.christmas_eve += 1;
        }
        if weekday == datetime.weekday() {
            output.weekday += 1;
        }
        if now < datetime {
            output.in_the_future += 1;
        }
        if input.0 & 1 == 1 {
            output.lsb_is_1 += 1;
        }
    }
    Json(output)
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use serde_json::json;
    use tokio::time::sleep;

    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        let server = routes_test().await;
        server
            .post("/12/save/packet20231212")
            .await
            .assert_status_ok();
        sleep(Duration::new(2, 0)).await;
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&2u64);
        sleep(Duration::new(2, 0)).await;
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&4u64);
        server
            .post("/12/save/packet20231212")
            .await
            .assert_status_ok();
        server
            .get("/12/load/packet20231212")
            .await
            .assert_json(&0u64);
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .await
            .post("/12/ulids")
            .json(&json!([
                "01BJQ0E1C3Z56ABCD0E11HYX4M",
                "01BJQ0E1C3Z56ABCD0E11HYX5N",
                "01BJQ0E1C3Z56ABCD0E11HYX6Q",
                "01BJQ0E1C3Z56ABCD0E11HYX7R",
                "01BJQ0E1C3Z56ABCD0E11HYX8P"
            ]))
            .await
            .assert_json(&json!([
                "015cae07-0583-f94c-a5b1-a070431f7516",
                "015cae07-0583-f94c-a5b1-a070431f74f8",
                "015cae07-0583-f94c-a5b1-a070431f74d7",
                "015cae07-0583-f94c-a5b1-a070431f74b5",
                "015cae07-0583-f94c-a5b1-a070431f7494"
            ]));
    }

    #[tokio::test]
    async fn task3() {
        routes_test()
            .await
            .post("/12/ulids/5")
            .json(&json!([
                "00WEGGF0G0J5HEYXS3D7RWZGV8",
                "76EP4G39R8JD1N8AQNYDVJBRCF",
                "018CJ7KMG0051CDCS3B7BFJ3AK",
                "00Y986KPG0AMGB78RD45E9109K",
                "010451HTG0NYWMPWCEXG6AJ8F2",
                "01HH9SJEG0KY16H81S3N1BMXM4",
                "01HH9SJEG0P9M22Z9VGHH9C8CX",
                "017F8YY0G0NQA16HHC2QT5JD6X",
                "03QCPC7P003V1NND3B3QJW72QJ"
            ]))
            .await
            .assert_json(&json!({
              "christmas eve": 3,
              "weekday": 1,
              "in the future": 2,
              "LSB is 1": 5
            }));
    }

    #[tokio::test]
    async fn extra1() {
        routes_test()
            .await
            .post("/12/ulids/0")
            .json(&json!([
                "00WEGGF0G0J5HEYXS3D7RWZGV8",
                "76EP4G39R8JD1N8AQNYDVJBRCF",
                "018CJ7KMG0051CDCS3B7BFJ3AK",
                "00Y986KPG0AMGB78RD45E9109K",
                "010451HTG0NYWMPWCEXG6AJ8F2",
                "01HH9SJEG0KY16H81S3N1BMXM4",
                "01HH9SJEG0P9M22Z9VGHH9C8CX",
                "017F8YY0G0NQA16HHC2QT5JD6X",
                "03QCPC7P003V1NND3B3QJW72QJ"
            ]))
            .await
            .assert_json(&json!({
                "christmas eve": 3,
                "weekday": 0,
                "in the future": 2,
                "LSB is 1": 5
            }));
    }

    #[tokio::test]
    async fn extra2() {
        routes_test()
            .await
            .post("/12/ulids/2")
            .json(&json!(["04BJK8N300BAMR9SQQWPWHVYKZ"]))
            .await
            .assert_json(&json!({
                "christmas eve": 1,
                "weekday": 1,
                "in the future": 1,
                "LSB is 1": 1
            }));
    }
}
