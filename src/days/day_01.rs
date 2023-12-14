use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/*numbers", get(numbers))
}

async fn numbers(Path(numbers): Path<String>) -> Json<i64> {
    numbers
        .trim_end_matches('/')
        .split('/')
        .map(|x| x.parse::<i64>().unwrap())
        .fold(0, std::ops::BitXor::bitxor)
        .pow(3)
        .into()
}

#[cfg(test)]
mod test {
    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test().await.get("/1/4/8").await.assert_json(&1728);
    }

    #[tokio::test]
    async fn task2_1() {
        routes_test().await.get("/1/10").await.assert_json(&1000);
    }

    #[tokio::test]
    async fn task2_2() {
        routes_test()
            .await
            .get("/1/4/5/8/10")
            .await
            .assert_json(&27);
    }
}
