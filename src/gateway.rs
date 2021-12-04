extern crate reqwest;
extern crate websockets;

use crate::{DiscordErr, Bot};

pub async fn resolve(bot: &Bot) -> Result<String, DiscordErr> {
    use std::collections::HashMap;

    let client = reqwest::Client::new();
    let req = client.get("https://discord.com/api/v9/gateway/bot");
    let res = req
        .header("Authorization", format!("Bot {}", bot.token))
        .send()
        .await;

    match res {
        Ok(res) => {
            let payload = res.text().await.unwrap();
            let payload: HashMap<&str, serde_json::Value> = serde_json::from_str(&payload).unwrap();
            match payload.get("url") {
                Some(serde_json::Value::String(url)) => {
                    let s = url.clone();
                    Ok(s)
                },
                _ => Err(DiscordErr)
            }
        },
        Err(_) => Err(DiscordErr)
    }
}

pub async fn connect(bot: &Bot, url: &str) {}

pub async fn identify() {}

pub async fn heartbeat() {}

pub async fn listen() {}
