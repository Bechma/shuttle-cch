use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::routing::post;
use axum::{Json, Router};
use minijinja::Environment;

type Day14State = Arc<Environment<'static>>;

pub(super) fn route() -> Router {
    let mut env = Environment::new();
    env.add_template(
        "unsafe",
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{ content | safe }}
  </body>
</html>"#,
    )
    .unwrap();
    env.add_template(
        "safe",
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{ content | escape }}
  </body>
</html>"#,
    )
    .unwrap();
    Router::new()
        .route("/unsafe", post(no_safe))
        .route("/safe", post(safe))
        .with_state(Arc::new(env))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Content {
    content: String,
}

async fn no_safe(State(env): State<Day14State>, Json(ctx): Json<Content>) -> Html<String> {
    env.get_template("unsafe")
        .unwrap()
        .render(ctx)
        .unwrap()
        .into()
}

async fn safe(State(env): State<Day14State>, Json(ctx): Json<Content>) -> Html<String> {
    env.get_template("safe")
        .unwrap()
        .render(ctx)
        .unwrap()
        .replace("&#x2f;", "/") // They don't want to escape "/"
        .into()
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[tokio::test]
    async fn task1() {
        super::super::routes_test()
            .await
            .post("/14/unsafe")
            .json(&json!({"content": "<h1>Welcome to the North Pole!</h1>"}))
            .await
            .assert_text(
                r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    <h1>Welcome to the North Pole!</h1>
  </body>
</html>"#,
            )
    }

    #[tokio::test]
    async fn task2() {
        super::super::routes_test()
            .await
            .post("/14/safe")
            .json(&json!({"content": "<script>alert(\"XSS Attack!\")</script>"}))
            .await
            .assert_text(
                r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    &lt;script&gt;alert(&quot;XSS Attack!&quot;)&lt;/script&gt;
  </body>
</html>"#,
            )
    }
}
