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
    const SHELF: &str = "shelf";
    const SHELF_TRIGGER: &str = "elf on a ";

    let (mut elf_on_a_shelf, mut shelf_with_no_elf_on_it) = (0, 0);
    for i in 0..=body.len() - SHELF.len() {
        if SHELF == &body[i..i + SHELF.len()] {
            if i >= SHELF_TRIGGER.len() && body[..i].ends_with(SHELF_TRIGGER) {
                elf_on_a_shelf += 1;
            } else {
                shelf_with_no_elf_on_it += 1;
            }
        }
    }
    Json(Elf {
        elf: body.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .await
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
            .await
            .post("/6")
            .text(
                "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.",
            )
            .await
            .assert_json(&json!({"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}));
    }

    #[tokio::test]
    async fn extra1() {
        routes_test()
            .await
            .post("/6")
            .text("elf elf elf on a shelf")
            .await
            .assert_json(&json!({
                "elf":4,
                "elf on a shelf":1,
                "shelf with no elf on it":0
            }));
    }

    #[tokio::test]
    async fn extra2() {
        routes_test()
            .await
            .post("/6")
            .text("In Belfast I heard an elf on a shelf on a shelf on a ")
            .await
            .assert_json(&json!({
                "elf":4,
                "elf on a shelf":2,
                "shelf with no elf on it":0
            }));
    }

    #[tokio::test]
    async fn extra3() {
        routes_test().await
            .post("/6")
            .text("Somewhere in Belfast under a shelf store but above the shelf realm there's an elf on a shelf on a shelf on a shelf on a elf on a shelf on a shelf on a shelf on a shelf on a elf on a elf on a elf on a shelf on a ")
            .await
            .assert_json(&json!({
                "elf":16,
                "elf on a shelf":8,
                "shelf with no elf on it":2
            }));
    }
}
