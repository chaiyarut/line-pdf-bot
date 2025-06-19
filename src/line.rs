use axum::{Json};
use serde_json::Value;
use reqwest::Client;
use std::{fs::File, io::Write};
use uuid::Uuid;
use crate::pdf::remove_page_2;

const CHANNEL_ACCESS_TOKEN: &str = "YOUR_LINE_CHANNEL_ACCESS_TOKEN";

pub async fn handle_webhook(Json(payload): Json<Value>) -> &'static str {
    if let Some(events) = payload["events"].as_array() {
        for event in events {
            if let Some(msg) = event["message"].as_object() {
                if msg["type"] == "file" && msg["fileName"].as_str().unwrap_or("").ends_with(".pdf") {
                    let message_id = msg["id"].as_str().unwrap();
                    let reply_token = event["replyToken"].as_str().unwrap();

                    let client = Client::new();
                    let pdf_bytes = crate::utils::download_line_file(&client, message_id).await.unwrap();

                    let uuid = Uuid::new_v4();
                    let temp_path = format!("static/{}_input.pdf", uuid);
                    let mut file = File::create(&temp_path).unwrap();
                    file.write_all(&pdf_bytes).unwrap();

                    let output_path = format!("static/{}.pdf", uuid);
                    remove_page_2(&temp_path, &output_path).unwrap();

                    let url = format!("https://your-domain.com/static/{}.pdf", uuid);
                    crate::utils::reply_text(&client, reply_token, &url).await.unwrap();
                }
            }
        }
    }
    "OK"
}
