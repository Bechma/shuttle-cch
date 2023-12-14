use axum::routing::post;
use axum::{Json, Router};

pub(super) fn route() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}

async fn strength(Json(payload): Json<Vec<Reindeer>>) -> Json<i64> {
    payload.into_iter().map(|x| x.strength).sum::<i64>().into()
}

#[derive(serde::Deserialize)]
struct Reindeer {
    #[serde(default)]
    name: String,
    strength: i64,
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    height: i64,
    #[serde(default)]
    antler_width: i64,
    #[serde(default)]
    snow_magic_power: i64,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
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
        return ContestOutput::default().into();
    };
    let (mut fastest, mut tallest, mut magician, mut consumer) = (first, first, first, first);
    for x in &payload {
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
    }
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

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .await
            .post("/4/strength")
            .json(&json!([
              { "name": "Dasher", "strength": 5 },
              { "name": "Dancer", "strength": 6 },
              { "name": "Prancer", "strength": 4 },
              { "name": "Vixen", "strength": 7 }
            ]))
            .await
            .assert_json(&22);
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .await
            .post("/4/contest")
            .json(&json!([
              {
                "name": "Dasher",
                "strength": 5,
                "speed": 50.4,
                "height": 80,
                "antler_width": 36,
                "snow_magic_power": 9001,
                "favorite_food": "hay",
                "cAnD13s_3ATeN-yesT3rdAy": 2
              },
              {
                "name": "Dancer",
                "strength": 6,
                "speed": 48.2,
                "height": 65,
                "antler_width": 37,
                "snow_magic_power": 4004,
                "favorite_food": "grass",
                "cAnD13s_3ATeN-yesT3rdAy": 5
              }
            ]))
            .await
            .assert_json(&json!({
              "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
              "tallest": "Dasher is standing tall with his 36 cm wide antlers",
              "magician": "Dasher could blast you away with a snow magic power of 9001",
              "consumer": "Dancer ate lots of candies, but also some grass"
            }));
    }
}
