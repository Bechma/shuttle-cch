use axum::routing::post;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/", post(elf_count))
}

#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(body: String) -> Json<Elf> {
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    let shelves = body.matches("shelf").count();
    Json(Elf {
        elf: body.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it: shelves - elf_on_a_shelf,
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .post("/6")
            .text(
                "The mischievous elf peeked out from behind the toy workshop,
and another elf joined in the festive dance.
Look, there is also an elf on that shelf!",
            )
            .await
            .assert_json(&json!({"elf":4,"elf on a shelf":0,"shelf with no elf on it":1}));
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .post("/6")
            .text(
                "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.",
            )
            .await
            .assert_json(&json!({"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}));
    }
}
