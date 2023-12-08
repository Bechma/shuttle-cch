use std::ops::Sub;

use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use base64::Engine;

const COOKIE_NAME: &str = "recipe";

pub(super) fn route() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq, serde::Deserialize, serde::Serialize)]
struct Recipe {
    #[serde(default, skip_serializing_if = "is_default")]
    flour: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    sugar: usize,
    #[serde(default, skip_serializing_if = "is_default")]
    butter: usize,
    #[serde(default, skip_serializing_if = "is_default", rename = "baking powder")]
    baking_powder: usize,
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "chocolate chips"
    )]
    chocolate_chips: usize,
}

impl Sub for Recipe {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            flour: self.flour - other.flour,
            sugar: self.sugar - other.sugar,
            butter: self.butter - other.butter,
            baking_powder: self.baking_powder - other.baking_powder,
            chocolate_chips: self.chocolate_chips - other.chocolate_chips,
        }
    }
}

impl std::ops::SubAssign for Recipe {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

fn is_default(t: &usize) -> bool {
    *t == 0
}

async fn decode(jar: CookieJar) -> Json<Recipe> {
    decode_cookie(jar).map(Json).unwrap()
}

#[derive(serde::Deserialize)]
struct BakeInput {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(serde::Serialize)]
struct BakeResult {
    cookies: usize,
    pantry: Recipe,
}

// TODO: Missing Task 3
async fn bake(jar: CookieJar) -> Json<BakeResult> {
    decode_cookie(jar)
        .map(|mut x: BakeInput| {
            let mut cookies = 0;
            while x.recipe < x.pantry {
                cookies += 1;
                x.pantry -= x.recipe;
            }
            Json(BakeResult {
                cookies,
                pantry: x.pantry,
            })
        })
        .unwrap()
}

fn decode_cookie<T: serde::de::DeserializeOwned>(jar: CookieJar) -> Option<T> {
    let recipe = jar.get(COOKIE_NAME)?;
    base64::engine::general_purpose::STANDARD
        .decode(recipe.value())
        .map(|x| serde_json::from_slice(&x).unwrap())
        .ok()
}

#[cfg(test)]
mod test {
    use axum_extra::extract::cookie::Cookie;
    use serde_json::json;

    use super::super::routes_test;
    use super::COOKIE_NAME;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .get("/7/decode")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            ))
            .await
            .assert_json(&json!({"flour":100,"chocolate chips":20}));
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
            ))
            .await
            .assert_json(&json!({
  "cookies": 4,
  "pantry": {
    "flour": 5,
    "sugar": 307,
    "butter": 2002,
    "baking powder": 825,
    "chocolate chips": 257
  }
}));
    }
}
