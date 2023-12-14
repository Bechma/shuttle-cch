use std::collections::HashMap;

use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use base64::Engine;

type Recipe = HashMap<String, usize>;

const COOKIE_NAME: &str = "recipe";

pub(super) fn route() -> Router {
    Router::new()
        .route("/decode", get(decode))
        .route("/bake", get(bake))
}

async fn decode(jar: CookieJar) -> Json<serde_json::Value> {
    decode_cookie(&jar).map(Json).unwrap()
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

async fn bake(jar: CookieJar) -> Json<BakeResult> {
    decode_cookie(&jar)
        .map(|mut x: BakeInput| {
            let mut cookies = usize::MAX;
            // locate amount of cookies available to make
            for (ingredient, recipe_amount) in &x.recipe {
                if recipe_amount == &0 {
                    continue;
                }
                if let Some(pantry_amount) = x.pantry.get(ingredient) {
                    cookies = (pantry_amount / recipe_amount).min(cookies);
                    if cookies == 0 {
                        // we can't make a single cookie if we don't have enough of one of the ingredients
                        return Json(BakeResult {
                            cookies,
                            pantry: x.pantry,
                        });
                    }
                } else {
                    // ingredient not found in the pantry? we can't make cookies
                    return Json(BakeResult {
                        cookies: 0,
                        pantry: x.pantry,
                    });
                }
            }
            // recalculate pantry based on the number of cookies to make
            for (ingredient, recipe_amount) in x.recipe {
                let amount_needed = recipe_amount * cookies;
                x.pantry
                    .entry(ingredient)
                    .and_modify(|x| *x -= amount_needed);
            }
            Json(BakeResult {
                cookies,
                pantry: x.pantry,
            })
        })
        .unwrap()
}

fn decode_cookie<T: serde::de::DeserializeOwned>(jar: &CookieJar) -> Option<T> {
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
            .await
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
        routes_test().await
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

    #[tokio::test]
    async fn task3() {
        routes_test().await
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==",
            ))
            .await
            .assert_json(&json!({
  "cookies": 0,
  "pantry": {
    "cobblestone": 64,
    "stick": 4
  }
}));
    }

    #[tokio::test]
    async fn extra1() {
        routes_test()
            .await
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsiY2hpY2tlbiI6MX0sInBhbnRyeSI6eyJjaGlja2VuIjowfX0=",
            ))
            .await
            .assert_json(&json!({
                "cookies": 0,
                "pantry": {
                    "chicken": 0,
                }
            }));
    }

    #[tokio::test]
    async fn extra2() {
        routes_test().await
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsiY29jb2EgYmVhbiI6MSwiY2hpY2tlbiI6MH0sInBhbnRyeSI6eyJjb2NvYSBiZWFuIjo1LCJjb3JuIjo1LCJjdWN1bWJlciI6MH19",
            ))
            .await
            .assert_json(&json!({
            "cookies": 5,
            "pantry": {
                "cocoa bean": 0,
                "corn": 5,
                "cucumber": 0,
            }
        }));
    }

    #[tokio::test]
    async fn extra3() {
        routes_test().await
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsiY29jb2EgYmVhbiI6MSwiY2hpY2tlbiI6MH0sInBhbnRyeSI6eyJjb2NvYSBiZWFuIjo1LCJjaGlja2VuIjowfX0=",
            ))
            .await
            .assert_json(&json!({
            "cookies": 5,
            "pantry": {
                "cocoa bean": 0,
                "chicken": 0,
            }
        }));
    }

    #[tokio::test]
    async fn extra4() {
        routes_test().await
            .get("/7/bake")
            .add_cookie(Cookie::new(
                COOKIE_NAME,
                "eyJyZWNpcGUiOnsiY29jb2EgYmVhbiI6MSwiY2hpY2tlbiI6MH0sInBhbnRyeSI6eyJjb2NvYSBiZWFuIjo1fX0=",
            ))
            .await
            .assert_json(&json!({
            "cookies": 5,
            "pantry": {
                "cocoa bean": 0,
            }
        }));
    }
}
