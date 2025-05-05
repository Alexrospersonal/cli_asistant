use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use reqwest;
use serde::{Deserialize, Serialize};
use colored::*;
use reqwest::Client;

pub async fn execute(query: Vec<String>) -> Result<(), Box<dyn Error>> {

    let url = build_url(&query);
    let client = build_client(Duration::from_secs(4))?;
    let search_response = send_request(url, client).await?;

    print_data(&search_response).await?;

    Ok(())
}

async fn send_request(url: String, client: Client) -> Result<SearchResponse, Box<dyn Error>> {
    let response = client.get(url).send().await?;

    let status = response.status();

    if !status.is_success() {
        let text = response.text().await?;
        return Err(format!("HTTP error: {}. Response: {}", status, text).into());
    }

    let parsed = response.json::<SearchResponse>().await?;

    Ok(parsed)
}

fn build_url(query: &Vec<String>) -> String {
    let full_query = query.join(" ");

    format!(
        "https://api.stackexchange.com/2.3/search/advanced?order=desc&sort=relevance&q={}&site=stackoverflow",
        urlencoding::encode(&full_query)
    )
}

fn build_client(duration: Duration) -> Result<Client, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("cli_assistant/0.01")
        .timeout(duration)
        .build()?;

    Ok(client)
}

async fn print_data(search_response: &SearchResponse) -> Result<(), Box<dyn Error>> {
    let mut output = io::stdout().lock();

    if search_response.items.is_empty() {
        writeln!(output, "No results found for your query.")?;
        return Ok(());
    }

    for (idx, item) in search_response.items.iter().enumerate() {
        writeln!(output, "{} title: {}", format!("🔎 {}.", idx + 1).yellow(), item.title.blue())?;
        writeln!(output, "{}", item.link.green())?;
        writeln!(output, " ")?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchResponse {
    items: Vec<SearchItem>
}
#[derive(Serialize, Deserialize, Debug)]
struct SearchItem {
    title: String,
    link: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_send_request_mock() {
        let mut server = mockito::Server::new_async().await;
        
        let mock = server.mock("GET", "/search/advanced").match_query(
            mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("q".into(), "rust".into()),
                mockito::Matcher::UrlEncoded("site".into(), "stackoverflow".into())
            ])
        ).with_status(200).with_header("content-type", "application/json").with_body(r#"
            {
                "items": [
                    {
                        "title": "How to borrow in Rust?",
                        "link": "https://stackoverflow.com/q/123456"
                    }
                ]
            }
        "#)
            .create();
        
        let client = build_client(Duration::from_secs(2)).unwrap();
        let url = format!("{}/search/advanced?q=rust&site=stackoverflow", &server.url());

        let response = send_request(url, client).await.unwrap();
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].title, "How to borrow in Rust?");
        
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_build_url_encoding() {
        let query = vec!["rust".to_string(), "borrow checker".to_string()];
        let url = build_url(&query);
        assert!(url.contains("rust%20borrow%20checker"));
        assert!(url.contains("site=stackoverflow"));
    }
}