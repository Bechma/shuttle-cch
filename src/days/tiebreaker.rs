#[derive(serde::Deserialize, Debug)]
struct GetData {
    name: String,
    email: String,
}

#[derive(serde::Serialize, Debug)]
struct PostData {
    name: String,
    email: String,
    message: String,
}

#[derive(Debug)]
struct State {
    client: reqwest::Client,
    deserializer: josekit::jwe::alg::rsaes::RsaesJweDecrypter,
}

impl State {
    fn new() -> Self {
        let mut auth = reqwest::header::HeaderValue::from_static("Bearer Dx3cC6VfP9qK2pO6");
        auth.set_sensitive(true);
        Self {
            client: reqwest::ClientBuilder::new()
                .default_headers(reqwest::header::HeaderMap::from_iter([(
                    reqwest::header::AUTHORIZATION,
                    auth,
                )]))
                .build()
                .expect("client should not errored"),
            deserializer: josekit::jwe::RSA_OAEP_256
                .decrypter_from_pem(include_bytes!("private_key.pem"))
                .unwrap(),
        }
    }
}

#[tracing::instrument(ret, skip(state))]
async fn step1(state: &State) -> Vec<GetData> {
    state
        .client
        .get("https://www.codehunt.rs/api/naughty")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .map(|x| {
            tracing::info!("{x}");
            let (res, header) = josekit::jwe::deserialize_json(&x, &state.deserializer).unwrap();
            tracing::info!("header: {header:?}");
            tracing::info!("res: {res:?}");
            serde_json::from_slice(&res).unwrap()
        })
        .unwrap()
}

#[tracing::instrument(skip(state, data))]
async fn step2(state: &State, data: Vec<GetData>) {
    let mut iter = data.into_iter().peekable();
    while iter.peek().is_some() {
        let mut post_data = Vec::with_capacity(10);
        for d in iter.by_ref().take(10) {
            tracing::info!("message: {d:?}");
            post_data.push(PostData {
                email: d.email,
                message: format!(
                    "Dear {}, it seems this year you've made it onto the Naughty List. But don't worry, everyone makes mistakes. This is a chance to learn and grow. We believe in you and can't wait to see the wonderful things you do next year! Keep shining, and remember, Santa's always cheering for you!",
                    d.name.split_ascii_whitespace().next().unwrap()),
                name: d.name });
        }
        tracing::info!(
            "res call: {}",
            state
                .client
                .post("https://www.codehunt.rs/api/naughty")
                .json(&post_data)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
        );
    }
}

#[tokio::test]
async fn steps() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init();
    let state = State::new();
    let res = step1(&state).await;
    println!("{res:?}");
    step2(&state, res).await;
}
