use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::error::Error;

pub const DEFAULT_OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

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

pub async fn fetch_suggestion_from_api(
    prompt: &str,
    api_url: &str,
) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    println!("🔐 Key: {}", &api_key[..5]);

    let client = Client::new();
    let res = client
        .post(api_url)
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
    use crate::services::openai::fetch_suggestion_from_api;

    const PROMPT: &str = "Please analyze the following code and provide detailed suggestions for improvements,\
        optimizations, or best practices. Do not modify the code itself, just give recommendations.: \n\n";

    #[tokio::test]
    async fn test_fetch_suggestion_from_api() {
        unsafe {
            std::env::set_var("OPENAI_API_KEY", "fakekey");
        }

        let mut server = mockito::Server::new_async().await;

        let _mock = server
            .mock("POST", "/v1/chat/completions")
            .match_header("authorization", "Bearer fakekey")
            .with_status(200)
            .with_body(
                r#"{
                "choices": [{
                    "message": {"content": "Mocked AI response"}
                }]
            }"#,
            )
            .create();

        let url = format!("{}/v1/chat/completions", server.url());
        let response = fetch_suggestion_from_api(PROMPT, &url).await.unwrap();
        assert_eq!(response, "Mocked AI response");
    }
}
