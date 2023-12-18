use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use sqlx::Row;

pub(super) fn route(pool: sqlx::SqlitePool) -> Router {
    Router::new()
        .route("/reset", post(reset))
        .route("/orders", post(super::day_13::insert_orders))
        .route("/regions", post(insert_regions))
        .route("/regions/total", get(total))
        .route("/regions/top_list/:number", get(top_list))
        .with_state(pool)
}

async fn reset(State(db): State<sqlx::SqlitePool>) {
    // No TRUNCATE TABLE in sqlite
    let mut tx = db.begin().await.unwrap();
    sqlx::query("DELETE FROM orders")
        .execute(tx.as_mut())
        .await
        .unwrap();
    sqlx::query("DELETE FROM regions")
        .execute(tx.as_mut())
        .await
        .unwrap();
    tx.commit().await.unwrap();
}

#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow)]
struct Region {
    id: i64,
    name: String,
}

async fn insert_regions(State(db): State<sqlx::SqlitePool>, Json(payload): Json<Vec<Region>>) {
    let mut tx = db.begin().await.unwrap();
    for region in &payload {
        sqlx::query("INSERT INTO regions (id, name) VALUES ($1, $2)")
            .bind(region.id)
            .bind(&region.name)
            .execute(tx.as_mut())
            .await
            .unwrap();
    }
    tx.commit().await.unwrap();
}

#[derive(serde::Serialize, sqlx::FromRow)]
struct Total {
    region: String,
    total: i64,
}

async fn total(State(db): State<sqlx::SqlitePool>) -> Json<Vec<Total>> {
    let total: Vec<Total> = sqlx::query_as(
        r"
        SELECT regions.name as region, SUM(quantity) as total
        FROM regions JOIN orders ON regions.id = orders.region_id
        GROUP BY name",
    )
    .fetch_all(&db)
    .await
    .unwrap();
    Json(total)
}

#[derive(serde::Serialize, sqlx::FromRow, Debug)]
struct TopList {
    region: String,
    top_gifts: Vec<String>,
}

#[tracing::instrument(ret, skip(db))]
async fn top_list(
    State(db): State<sqlx::SqlitePool>,
    Path(number): Path<i64>,
) -> Json<Vec<TopList>> {
    if number < 1 {
        return Json(
            sqlx::query("SELECT name as region FROM regions")
                .map(|x: sqlx::sqlite::SqliteRow| {
                    let region: String = x.get("region");
                    TopList {
                        region,
                        top_gifts: vec![],
                    }
                })
                .fetch_all(&db)
                .await
                .unwrap(),
        );
    }

    // Just for fun, this is for postgres:
    //
    // SELECT region, array_remove(array_agg(gift_name), NULL) as top_gifts FROM (
    //     SELECT regions.name as region, gift_name, row_number() OVER (PARTITION BY regions.name order by regions.name ASC, SUM(quantity) DESC, gift_name ASC) as row_num
    //     FROM regions LEFT OUTER JOIN orders ON regions.id = orders.region_id
    //     GROUP BY regions.name, gift_name
    //     ORDER BY regions.name ASC, SUM(quantity) DESC, gift_name ASC
    //   ) as deep
    // where row_num <= 2
    // group by region;
    //
    //
    // Solved in SurrealDB for even more fun:
    //
    // SELECT name as region, array::slice((
    //     SELECT region_id, gift_name, math::sum(quantity) as q
    //     FROM orders
    //     WHERE region_id=$parent.id
    //     GROUP BY region_id, gift_name
    //     ORDER BY q desc
    // ).gift_name, 0, 2) as top_gift FROM regions
    // ORDER BY region;
    sqlx::query(
        r"
SELECT region, group_concat(gift_name, ', ') as top_gifts FROM (
  SELECT regions.name as region, gift_name, row_number() OVER (PARTITION BY regions.name order by regions.name ASC, SUM(quantity) DESC, gift_name ASC) as row_num
  FROM orders LEFT FULL JOIN regions ON regions.id = orders.region_id
  GROUP BY regions.name, gift_name
  ORDER BY regions.name ASC, SUM(quantity) DESC, gift_name ASC
)
where row_num <= $1
group by region;",
    ).bind(number)
    .map(|row: sqlx::sqlite::SqliteRow| {
        let region: String = row.get("region");
        let top_gifts: Option<String> = row.get("top_gifts");
        tracing::info!(?region, ?top_gifts);
        TopList {
            region,
            top_gifts: top_gifts.map(|x| x.split(", ").map(ToString::to_string).collect()).unwrap_or_default(),
        }
    })
    .fetch_all(&db)
    .await.map(|x| Json(x.into_iter().filter(|y| !y.region.is_empty()).collect()))
    .unwrap()
}

#[cfg(test)]
mod test {
    use serde_json::json;

    #[tokio::test]
    async fn all() {
        let server = super::super::routes_test().await;

        // Task 1
        server.post("/18/reset").await.assert_status_ok();
        server
            .post("/18/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Board Game","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Origami Set","quantity":8},
              {"id":3,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":4,"region_id":4,"gift_name":"Teddy Bear","quantity":10},
              {"id":5,"region_id":2,"gift_name":"Yarn Ball","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Art Set","quantity":3},
              {"id":7,"region_id":5,"gift_name":"Robot Lego Kit","quantity":5},
              {"id":8,"region_id":6,"gift_name":"Drone","quantity":9}
            ]))
            .await
            .assert_status_ok();
        server
            .post("/18/regions")
            .json(&json!([
              {"id":1,"name":"North Pole"},
              {"id":2,"name":"Europe"},
              {"id":3,"name":"North America"},
              {"id":4,"name":"South America"},
              {"id":5,"name":"Africa"},
              {"id":6,"name":"Asia"},
              {"id":7,"name":"Oceania"}
            ]))
            .await
            .assert_status_ok();
        server.get("/18/regions/total").await.assert_json(&json!([
          {"region":"Africa","total":5},
          {"region":"Asia","total":9},
          {"region":"Europe","total":19},
          {"region":"North America","total":15},
          {"region":"South America","total":10}
        ]));

        // Task 2
        server.post("/18/reset").await.assert_status_ok();
        server
            .post("/18/regions")
            .json(&json!([
              {"id":1,"name":"North Pole"},
              {"id":2,"name":"South Pole"},
              {"id":3,"name":"Kiribati"},
              {"id":4,"name":"Baker Island"}
            ]))
            .await
            .assert_status_ok();
        server
            .post("/18/orders")
            .json(&json!([
              {"id":1,"region_id":2,"gift_name":"Toy Train","quantity":5},
              {"id":2,"region_id":2,"gift_name":"Toy Train","quantity":3},
              {"id":3,"region_id":2,"gift_name":"Doll","quantity":8},
              {"id":4,"region_id":3,"gift_name":"Toy Train","quantity":3},
              {"id":5,"region_id":2,"gift_name":"Teddy Bear","quantity":6},
              {"id":6,"region_id":3,"gift_name":"Action Figure","quantity":12},
              {"id":7,"region_id":4,"gift_name":"Board Game","quantity":10},
              {"id":8,"region_id":3,"gift_name":"Teddy Bear","quantity":1},
              {"id":9,"region_id":3,"gift_name":"Teddy Bear","quantity":2}
            ]))
            .await
            .assert_status_ok();

        server
            .get("/18/regions/top_list/2")
            .await
            .assert_json(&json!([
              {"region":"Baker Island","top_gifts":["Board Game"]},
              {"region":"Kiribati","top_gifts":["Action Figure","Teddy Bear"]},
              {"region":"North Pole","top_gifts":[]},
              {"region":"South Pole","top_gifts":["Doll","Toy Train"]}
            ]));
    }
}

// Test SQL
// CREATE TABLE orders
// (
//     id        INT PRIMARY KEY,
//     region_id INT,
//     gift_name VARCHAR(50),
//     quantity  INT
// );
// CREATE TABLE regions
// (
//     id   INT PRIMARY KEY,
//     name VARCHAR(50)
// );
// INSERT INTO orders (id, region_id, gift_name, quantity) VALUES
// (1, 2, 'Toy Train', 5),
// (2, 2, 'Toy Train', 3),
// (3, 2, 'Doll', 8),
// (4, 3, 'Toy Train', 3),
// (5, 2, 'Teddy Bear', 6),
// (6, 3, 'Action Figure', 12),
// (7, 4, 'Board Game', 10),
// (8, 3, 'Teddy Bear', 1),
// (9, 3, 'Toy Train', 2);
//
// INSERT INTO regions (id, name) VALUES
// (1, 'North Pole'),
// (2, 'South Pole'),
// (3, 'Kiribati'),
// (4, 'Baker Island');
