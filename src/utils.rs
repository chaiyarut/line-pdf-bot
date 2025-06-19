use reqwest::Client;
use serde_json::json;

const CHANNEL_ACCESS_TOKEN: &str = "YOUR_LINE_CHANNEL_ACCESS_TOKEN";

pub async fn download_line_file(client: &Client, message_id: &str) -> Result<Vec<u8>, reqwest::Error> {
    let url = format!("https://api-data.line.me/v2/bot/message/{}/content", message_id);
    let res = client.get(url)
        .bearer_auth(CHANNEL_ACCESS_TOKEN)
        .send()
        .await?;

    Ok(res.bytes().await?.to_vec())
}

pub async fn reply_text(client: &Client, reply_token: &str, text: &str) -> Result<(), reqwest::Error> {
    let body = json!({
        "replyToken": reply_token,
        "messages": [
            {
                "type": "text",
                "text": text
            }
        ]
    });

    client.post("https://api.line.me/v2/bot/message/reply")
        .bearer_auth(CHANNEL_ACCESS_TOKEN)
        .json(&body)
        .send()
        .await?;

    Ok(())
}
