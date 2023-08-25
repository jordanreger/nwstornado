use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct BskyUser {
    identifier: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct PostRecord {
    #[serde(rename = "Authorization")]
    authorization: String,
    repo: String,
    collection: String,
    record: Record,
}

#[derive(Debug, Serialize)]
struct Record {
    text: String,
    #[serde(rename = "createdAt")]
    created_at: DateTime<Utc>,
}

pub async fn post_to_bluesky(warning_text: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let auth = BskyUser {
        identifier: dotenv!("NWSTORNADO_IDENTIFIER").into(),
        password: dotenv!("NWSTORNADO_PASSWORD").into(),
    };

    let session = client
        .post("https://bsky.social/xrpc/com.atproto.server.createSession")
        .header("User-Agent", "nwstornado.bsky.social")
        .json(&auth)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let auth_token = &session["accessJwt"].to_string().replace("\"", "");

    let now = Utc::now();
    let body = Record {
        text: warning_text,
        created_at: now,
    };

    let post = PostRecord {
        authorization: "Bearer ".to_string() + auth_token,
        repo: "nwstornado.bsky.social".to_string(),
        collection: "app.bsky.feed.post".to_string(),
        record: body,
    };

    let post_send = client
        .post("https://bsky.social/xrpc/com.atproto.repo.createRecord?repo=nwstornado.bsky.social")
        .bearer_auth(auth_token)
        .header("User-Agent", "nwstornado.bsky.social")
        .json(&post)
        .send()
        .await?;

    /* the following lines are only for debugging via text

    let post_send_res = post_send.text().await?;

    println!("{:?}", post_send_res);

    */

    Ok(())
}
