use axum::routing::post;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}

async fn strength(Json(payload): Json<Vec<Reindeer>>) -> Json<i64> {
    Json(payload.into_iter().map(|x| x.strength).sum())
}

#[derive(serde::Deserialize)]
struct Reindeer {
    name: String,
    strength: i64,
    speed: f64,
    height: i64,
    antler_width: i64,
    snow_magic_power: i64,
    favorite_food: String,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies: i64,
}

#[derive(Default, serde::Serialize)]
struct ContestOutput {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(Json(payload): Json<Vec<Reindeer>>) -> Json<ContestOutput> {
    let Some(first) = payload.first() else {
        return Json(ContestOutput::default());
    };
    let (mut fastest, mut tallest, mut magician, mut consumer) = (first, first, first, first);
    payload.iter().for_each(|x| {
        if fastest.speed < x.speed {
            fastest = x;
        }
        if tallest.height < x.height {
            tallest = x;
        }
        if magician.snow_magic_power < x.snow_magic_power {
            magician = x;
        }
        if consumer.candies < x.candies {
            consumer = x;
        }
    });
    Json(ContestOutput {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name,
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width,
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power,
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food,
        ),
    })
}
