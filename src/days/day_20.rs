use std::collections::HashMap;
use std::io::Read;

use axum::body::Bytes;
use axum::routing::post;
use axum::Router;

pub(super) fn route() -> Router {
    Router::new()
        .route("/archive_files", post(archive_files))
        .route("/archive_files_size", post(archive_files_size))
        .route("/cookie", post(cookie))
}

#[tracing::instrument(ret, skip(payload))]
async fn archive_files(payload: Bytes) -> String {
    tar::Archive::new(payload.as_ref())
        .entries()
        .unwrap()
        .count()
        .to_string()
}

#[tracing::instrument(ret, skip(payload))]
async fn archive_files_size(payload: Bytes) -> String {
    tar::Archive::new(payload.as_ref())
        .entries()
        .unwrap()
        .map(|x| x.unwrap().size())
        .sum::<u64>()
        .to_string()
}

#[tracing::instrument(ret, skip(payload))]
async fn cookie(payload: Bytes) -> String {
    // Send the repository to memory
    let hm: HashMap<String, Vec<u8>> = tar::Archive::new(payload.as_ref())
        .entries()
        .unwrap()
        .map(|x| {
            let x = x.unwrap();
            (
                x.path().unwrap().to_str().unwrap().to_string(),
                x.bytes().map(Result::unwrap).collect(),
            )
        })
        .collect();
    // Let's go to the commit history of christmas from the newest to the oldest
    for commit_line in std::str::from_utf8(&hm[".git/logs/refs/heads/christmas"])
        .unwrap()
        .lines()
        .rev()
    {
        // Parse each of the commit lines
        tracing::info!(commit_line);
        let res: Vec<&str> = commit_line.split_ascii_whitespace().take(3).collect();
        let (commit, user) = (res[1], res[2]);
        tracing::info!(commit, user);
        // Get the commit contents
        let commit_content = decode_obj(&hm, commit);
        let commit_content = std::str::from_utf8(&commit_content).unwrap();
        tracing::info!(commit_content);
        // From the commit contents, extract the tree hash to get the whole changelog
        let tree = commit_content
            .lines()
            .map(|x| x.split_ascii_whitespace().next_back().unwrap())
            .next()
            .unwrap();

        // Reverse the tree, and if COOKIE with commit hash is found, return the username and commit hash
        if decode_tree(&hm, tree) {
            return format!("{user} {commit}");
        }
    }
    "not found".into()
}

#[inline]
fn decode_tree(hm: &HashMap<String, Vec<u8>>, obj: &str) -> bool {
    for entry in Tree::from(decode_obj(hm, obj)).entries {
        if matches!(entry.mode, Mode::Tree) && decode_tree(hm, &entry.hash) {
            return true;
        }
        if entry.filename != "santa.txt" {
            continue;
        }
        let blob = decode_obj(hm, &entry.hash);
        let blob = std::str::from_utf8(blob.as_slice()).unwrap();
        if blob.contains("COOKIE") {
            return true;
        }
    }
    false
}

#[derive(Debug)]
enum Mode {
    Tree,
    Whatever,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        if value == b'4' {
            Self::Tree
        } else {
            Self::Whatever
        }
    }
}

#[derive(Debug)]
struct Entry {
    mode: Mode,
    filename: String,
    hash: String,
}

#[derive(Debug)]
struct Tree {
    entries: Vec<Entry>,
}

impl From<Vec<u8>> for Tree {
    /// Parse a raw bytes tree object
    #[tracing::instrument(ret)]
    fn from(value: Vec<u8>) -> Self {
        let mut res = Self { entries: vec![] };
        // We don't actually care about `tree 12345X\0.....`
        // So I trim it `tree 12345X\0`
        let mut iter = value
            .into_iter()
            .skip_while(|&x| x != 0) // skip tree 12345X
            .skip(1) // skip \0
            .peekable();
        while iter.peek().is_some() {
            let mode = Mode::from(iter.next().expect("there should be a mode"));
            let filename: String = iter
                .by_ref()
                .skip_while(|x| x != &b' ')
                .take_while(|x| x != &b'\x00')
                .map(char::from)
                .skip(1)
                .collect();
            let hash: String = iter.by_ref().take(20).fold(String::new(), |mut acc, x| {
                acc.push_str(&format!("{x:02x}"));
                acc
            });

            res.entries.push(Entry {
                mode,
                filename,
                hash,
            });
        }
        res
    }
}

#[tracing::instrument(skip(hm))]
fn decode_obj(hm: &HashMap<String, Vec<u8>>, obj: &str) -> Vec<u8> {
    let content = &hm[&format!(".git/objects/{}/{}", &obj[..2], &obj[2..])];
    tracing::info!(content_len = content.len());
    let mut res = Vec::new();
    flate2::read::ZlibDecoder::new(&content[..])
        .read_to_end(&mut res)
        .unwrap();
    res
}

#[cfg(test)]
mod test {
    use crate::days::routes_test;
    use axum::body::Bytes;

    #[tokio::test]
    async fn task1() {
        let north_pole = Bytes::from_static(include_bytes!("../../assets/northpole20231220.tar"));
        let server = routes_test().await;
        server
            .post("/20/archive_files")
            .bytes(north_pole.clone())
            .await
            .assert_text("6");
        server
            .post("/20/archive_files_size")
            .bytes(north_pole)
            .await
            .assert_text("1196282");
    }

    #[tokio::test]
    async fn task2_1() {
        let c = Bytes::from_static(include_bytes!("../../assets/cookiejar.tar"));
        routes_test()
            .await
            .post("/20/cookie")
            .bytes(c)
            .await
            .assert_text("Grinch 71dfab551a1958b35b7436c54b7455dcec99a12c");
    }

    #[tokio::test]
    async fn task2_2() {
        let c = Bytes::from_static(include_bytes!("../../assets/lottery.tar"));
        routes_test()
            .await
            .post("/20/cookie")
            .bytes(c)
            .await
            .assert_text("elf-27221 6342c1dbdb560f0d5dcaac7566fca51454866664");
    }
}
