use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use axum::routing::post;
use axum::Router;

pub(super) fn route() -> Router {
    Router::new()
        .route("/integers", post(integers))
        .route("/rocket", post(rocket))
}

#[tracing::instrument]
async fn integers(payload: String) -> String {
    let mut memory = HashSet::new();
    let mut dup = HashSet::new();
    for line in payload.lines() {
        if !memory.insert(line) {
            dup.insert(line);
        }
    }
    "🎁".repeat(
        memory
            .difference(&dup)
            .map(|x| {
                tracing::info!("FOUND: {x}");
                x.parse::<usize>().unwrap()
            })
            .sum(),
    )
}

#[tracing::instrument(ret, skip(payload))]
async fn rocket(payload: String) -> String {
    let mut lines = payload.lines();
    let number_of_stars = lines.next().map(|x| x.parse().unwrap()).unwrap();
    let stars: Vec<Vec<i32>> = lines
        .by_ref()
        .take(number_of_stars)
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let number_of_portals = lines.next().unwrap().parse().unwrap();
    let mut portal_paths = HashMap::new();
    for p in lines.take(number_of_portals).map(|x| {
        x.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
    }) {
        portal_paths.entry(p[0]).or_insert(Vec::new()).push(p);
    }
    let goal = stars.len() - 1;

    let mut number_steps = usize::MAX;
    let mut distance = f32::MAX;
    tracing::info!("{portal_paths:?}");

    for i in &portal_paths[&0] {
        let Some(aux) = pathfinding::directed::bfs::bfs(
            i,
            |x| {
                tracing::info!("searching... {x:?}");
                portal_paths.get(&x[1]).map(Vec::clone).unwrap_or_default()
            },
            |x| x[1] == goal,
        ) else {
            tracing::warn!("No path available starting from {i:?}");
            continue;
        };
        tracing::info!("path available! {aux:?}");
        match aux.len().cmp(&number_steps) {
            Ordering::Less => {
                number_steps = aux.len();
                distance = aux
                    .into_iter()
                    .map(|x| calculate_distance(&stars[x[0]], &stars[x[1]]))
                    .sum();
            }
            Ordering::Equal => {
                let aux = aux
                    .into_iter()
                    .map(|x| calculate_distance(&stars[x[0]], &stars[x[1]]))
                    .sum();
                distance = distance.min(aux);
            }
            Ordering::Greater => continue,
        }
    }

    format!("{number_steps} {distance:.03}")
}

/// We need to convert to f32 because of a precision error from the validator
#[allow(clippy::cast_precision_loss)]
fn calculate_distance(p1: &[i32], p2: &[i32]) -> f32 {
    (((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2)) as f32).sqrt()
}

#[cfg(test)]
mod test {
    use super::super::routes_test;

    #[tokio::test]
    async fn task1() {
        routes_test()
            .await
            .post("/22/integers")
            .text(
                "888
77
888
22
77
",
            )
            .await
            .assert_text("🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁");
    }

    #[tokio::test]
    async fn task2() {
        routes_test()
            .await
            .post("/22/rocket")
            .text(
                "5
0 1 0
-2 2 3
3 -3 -5
1 1 5
4 3 5
4
0 1
2 4
3 4
1 2
",
            )
            .await
            .assert_text("3 26.123");
    }
}
