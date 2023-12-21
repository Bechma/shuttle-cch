use axum::extract::Query;
use axum::Json;

#[derive(serde::Deserialize, Debug)]
pub(super) struct Pagination {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

#[tracing::instrument(ret)]
pub(super) async fn day(
    Query(q): Query<Pagination>,
    Json(payload): Json<Vec<String>>,
) -> Json<Vec<serde_json::Value>> {
    let limit = q.limit.unwrap_or(payload.len());
    let mut iter = payload.into_iter().skip(q.offset).take(limit);

    Json(match q.split {
        Some(split) => (0..limit.div_ceil(split))
            .map(|_| iter.by_ref().take(split).collect())
            .collect(),
        None => iter.map(|x| serde_json::to_value(x).unwrap()).collect(),
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::super::routes_test;

    const INPUT: &[&str] = &[
        "Ava", "Caleb", "Mia", "Owen", "Lily", "Ethan", "Zoe", "Nolan", "Harper", "Lucas",
        "Stella", "Mason", "Olivia",
    ];

    #[tokio::test]
    async fn task1() {
        routes_test()
            .await
            .post("/5")
            .add_query_param("offset", 3)
            .add_query_param("limit", 5)
            .json(&json!(INPUT))
            .await
            .assert_json(&json!(["Owen", "Lily", "Ethan", "Zoe", "Nolan"]));
    }

    #[tokio::test]
    async fn task2() {
        let server = routes_test().await;
        server
            .post("/5")
            .add_query_param("split", 4)
            .json(&json!(INPUT))
            .await
            .assert_json(&json!([
                ["Ava", "Caleb", "Mia", "Owen"],
                ["Lily", "Ethan", "Zoe", "Nolan"],
                ["Harper", "Lucas", "Stella", "Mason"],
                ["Olivia"]
            ]));
        server
            .post("/5")
            .add_query_param("offset", 5)
            .add_query_param("split", 2)
            .json(&json!(INPUT))
            .await
            .assert_json(&json!([
                ["Ethan", "Zoe"],
                ["Nolan", "Harper"],
                ["Lucas", "Stella"],
                ["Mason", "Olivia"]
            ]));
    }
}
