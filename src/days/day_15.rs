use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use std::iter::zip;

pub(super) fn route() -> Router {
    Router::new()
        .route("/nice", post(nice))
        .route("/game", post(game))
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Nice {
    input: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Result {
    #[serde(rename = "nice")]
    Nice,
    #[serde(rename = "naughty")]
    Naughty,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct NiceOutput {
    result: Result,
}

fn extract(mut payload: serde_json::Value) -> Option<String> {
    payload.as_object_mut()?.remove("input")?.to_string().into()
}

#[tracing::instrument(ret)]
async fn nice(Json(payload): Json<serde_json::Value>) -> (StatusCode, Json<NiceOutput>) {
    const FORBIDDEN: &[&str] = &["ab", "cd", "pq", "xy"];
    const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];
    let Some(input) = extract(payload) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(NiceOutput {
                result: Result::Naughty,
            }),
        );
    };

    let mut vowels = 0;
    let mut consecutive = false;
    for i in 0..input.len() - 1 {
        let w = &input[i..i + 2];
        let mut c = w.chars();
        let (char_1, char_2) = (c.next().unwrap(), c.next().unwrap());
        if char_1.is_alphabetic() && char_1 == char_2 {
            consecutive = true;
        }
        if VOWELS.iter().any(|&x| x == char_1) {
            vowels += 1;
        }
        if FORBIDDEN.iter().any(|&x| x == w) {
            return (
                StatusCode::BAD_REQUEST,
                Json(NiceOutput {
                    result: Result::Naughty,
                }),
            );
        }
    }

    if consecutive && vowels >= 3 {
        (
            StatusCode::OK,
            Json(NiceOutput {
                result: Result::Nice,
            }),
        )
    } else {
        (
            StatusCode::BAD_REQUEST,
            Json(NiceOutput {
                result: Result::Naughty,
            }),
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Task2 {
    result: Result,
    reason: Reason,
}

impl Task2 {
    #[inline]
    fn nice(reason: Reason) -> Self {
        Self {
            result: Result::Nice,
            reason,
        }
    }
    #[inline]
    fn naughty(reason: Reason) -> Self {
        Self {
            result: Result::Naughty,
            reason,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Reason {
    BadRequest,
    #[serde(rename = "8 chars")]
    Rule1,
    #[serde(rename = "more types of chars")]
    Rule2,
    #[serde(rename = "55555")]
    Rule3,
    #[serde(rename = "math is hard")]
    Rule4,
    #[serde(rename = "not joyful enough")]
    Rule5,
    #[serde(rename = "illegal: no sandwich")]
    Rule6,
    #[serde(rename = "outranged")]
    Rule7,
    #[serde(rename = "😳")]
    Rule8,
    #[serde(rename = "not a coffee brewer")]
    Rule9,
    #[serde(rename = "that's a nice password")]
    Success,
}

impl IntoResponse for Reason {
    fn into_response(self) -> Response {
        match self {
            Reason::BadRequest | Reason::Rule1 | Reason::Rule2 | Reason::Rule3 | Reason::Rule4 => {
                (StatusCode::BAD_REQUEST, Json(Task2::naughty(self)))
            }
            Reason::Rule5 => (StatusCode::NOT_ACCEPTABLE, Json(Task2::naughty(self))),
            Reason::Rule6 => (
                StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                Json(Task2::naughty(self)),
            ),
            Reason::Rule7 => (
                StatusCode::RANGE_NOT_SATISFIABLE,
                Json(Task2::naughty(self)),
            ),
            Reason::Rule8 => (StatusCode::UPGRADE_REQUIRED, Json(Task2::naughty(self))),
            Reason::Rule9 => (StatusCode::IM_A_TEAPOT, Json(Task2::naughty(self))),
            Reason::Success => (StatusCode::OK, Json(Task2::nice(self))),
        }
        .into_response()
    }
}

#[tracing::instrument(ret)]
async fn game(Json(payload): Json<serde_json::Value>) -> Reason {
    let Some(input) = extract(payload) else {
        return Reason::BadRequest;
    };
    if input.len() < 8 {
        return Reason::Rule1;
    }

    let (mut upper, mut lower, mut numeric) = (false, false, 0);
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            upper = true;
        } else if c.is_ascii_lowercase() {
            lower = true;
        } else if c.is_ascii_digit() {
            numeric += 1;
        }
    }
    if !(upper && lower && numeric > 0) {
        return Reason::Rule2;
    }
    if numeric < 5 {
        return Reason::Rule3;
    }
    let rule_acc = input.chars().fold((0, 0), |(acc, digits), ch| {
        let Some(d) = ch.to_digit(10) else {
            return (acc + digits, 0);
        };
        (acc, digits * 10 + d)
    });
    tracing::debug!("Rule4 input: {rule_acc:?}");
    if rule_acc.0 + rule_acc.1 != 2023 {
        return Reason::Rule4;
    }
    if input
        .chars()
        .try_fold((false, false, false), |(j, o, y), c| match c {
            'j' => {
                if j || o || y {
                    None
                } else {
                    Some((true, o, y))
                }
            }
            'o' => {
                if !j || o || y {
                    None
                } else {
                    Some((j, true, y))
                }
            }
            'y' => {
                if !j || !o || y {
                    None
                } else {
                    Some((j, o, true))
                }
            }
            _ => Some((j, o, y)),
        })
        .is_none()
    {
        return Reason::Rule5;
    }
    if !zip(
        zip(input.chars(), input.chars().skip(1)),
        input.chars().skip(2),
    )
    .any(|((c1, c2), c3)| c1.is_ascii_alphabetic() && c1 == c3 && c1 != c2)
    {
        return Reason::Rule6;
    }
    if !input
        .chars()
        .any(|c| ('\u{2980}'..='\u{2BFF}').contains(&c))
    {
        return Reason::Rule7;
    }
    if !input.chars().any(unic_emoji_char::is_emoji_presentation) {
        return Reason::Rule8;
    }
    if !sha256::digest(input).ends_with('a') {
        return Reason::Rule9;
    }
    Reason::Success
}

#[cfg(test)]
mod test {
    use axum::http::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn task1() {
        let server = super::super::routes_test().await;
        let nice = server
            .post("/15/nice")
            .json(&json!({"input": "hello there"}))
            .expect_success()
            .await;
        nice.assert_status_ok();
        nice.assert_json(&json!({"result":"nice"}));
        let naughty = server
            .post("/15/nice")
            .json(&json!({"input": "abcd"}))
            .expect_failure()
            .await;
        naughty.assert_status_bad_request();
        naughty.assert_json(&json!({"result":"naughty"}));
        server
            .post("/15/nice")
            .json(&"{Grinch? GRINCH!}")
            .expect_failure()
            .await
            .assert_status_bad_request();
    }

    #[tokio::test]
    async fn task2() {
        let server = super::super::routes_test().await;
        let t1 = server
            .post("/15/game")
            .json(&json!({"input": "password"}))
            .expect_failure()
            .await;
        t1.assert_status_bad_request();
        t1.assert_json(&json!({"result":"naughty","reason":"more types of chars"}));
        let t2 = server
            .post("/15/game")
            .json(&json!({"input": "Password12345"}))
            .expect_failure()
            .await;
        t2.assert_status_bad_request();
        t2.assert_json(&json!({"result":"naughty","reason":"math is hard"}));
        let t3 = server
            .post("/15/game")
            .json(&json!({"input": "23jPassword2000y"}))
            .expect_failure()
            .await;
        t3.assert_status(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS);
        t3.assert_json(&json!({"result":"naughty","reason":"illegal: no sandwich"}));
    }
}
