use axum::routing::post;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new().route("/", post(elf_count))
}

#[derive(serde::Serialize)]
struct Elf {
    elf: usize,
    elf_on_a_shelf: usize,
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(body: String) -> Json<Elf> {
    let elf_on_a_shelf = body.matches("elf on a shelf").count();
    Json(Elf {
        elf: body.matches("elf").count(),
        elf_on_a_shelf,
        shelf_with_no_elf_on_it: body.matches("shelf").count() - elf_on_a_shelf,
    })
}
