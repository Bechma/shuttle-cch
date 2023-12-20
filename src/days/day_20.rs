use std::collections::HashMap;
use std::io::Read;

use axum::body::Bytes;
use axum::routing::post;
use axum::Router;
use git_object::bstr::ByteSlice;

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
    for commit_line in hm[".git/logs/refs/heads/christmas"]
        .to_str()
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
        let commit_content = commit_content.to_str().unwrap();
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
    // The library doesn't accept the common format `tree 12345X\0.....`
    // So I need to manually trim `tree 12345X\0`
    let tree: Vec<_> = decode_obj(hm, obj)
        .into_iter()
        .skip_while(|&x| x != 0) // skip tree 12345X
        .skip(1) // skip \0
        .collect();

    let tree = git_object::TreeRef::from_bytes(tree.as_slice()).unwrap();
    for entry in tree.entries {
        if entry.mode == git_object::tree::EntryMode::Tree
            && decode_tree(hm, &entry.oid.to_string())
        {
            return true;
        }
        if entry.filename != "santa.txt" {
            continue;
        }
        if decode_obj(hm, &entry.oid.to_string()).contains_str("COOKIE") {
            return true;
        }
    }
    false
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
