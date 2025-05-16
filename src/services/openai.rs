use dotenvy::dotenv;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

pub async fn fetch_suggestion_from_api(prompt: &str) -> Result<String, Box<dyn Error>> {
    let env = dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY")?;
    println!("🔐 Key: {}", &api_key[..5]);

    // TODO: Далі — HTTP запит
    let client = Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&json!({
            "model": "gpt-4.1-mini",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant for analyzing and improving Rust code."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await?;

    // let body = res.text().await?;

    let body = res.json::<ChatResponse>().await?;
    let answer = body
        .choices
        .get(0)
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "No response from model.".to_string());

    Ok(answer)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fetch_suggestion_from_api() {}
}
