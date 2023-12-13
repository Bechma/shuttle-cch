use image::GenericImageView;

pub(super) fn route() -> axum::Router {
    axum::Router::new()
        .route("/red_pixels", axum::routing::post(magical))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
}

async fn magical(mut multipart: axum::extract::Multipart) -> axum::Json<usize> {
    let Some(field) = multipart.next_field().await.unwrap() else {
        return axum::Json(0);
    };

    let img = image::load_from_memory(field.bytes().await.unwrap().as_ref()).unwrap();
    axum::Json(
        img.pixels()
            .filter(|x| {
                let [r, g, b, _] = x.2 .0;
                u16::from(r) > (u16::from(g) + u16::from(b))
            })
            .count(),
    )
}

#[cfg(test)]
mod test {
    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        let response = routes_test().get("/11/assets/decoration.png").await;
        assert_eq!(
            response
                .header(axum_test::http::header::CONTENT_TYPE)
                .to_str()
                .unwrap(),
            "image/png"
        );
        assert_eq!(
            response
                .header(axum_test::http::header::CONTENT_LENGTH)
                .to_str()
                .unwrap(),
            "787297"
        );
    }

    // TODO: not sure how to test this one with axum-test
    // #[tokio::test]
    // async fn task2() {
    //     let res: f64 = routes_test().post("/11/red_pixels").form().await.json();
    // }
}
