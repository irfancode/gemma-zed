use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<i32>,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct LmStudioResponse {
    choices: Option<Vec<Choice>>,
    #[serde(rename = "error")]
    error: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Option<Message>,
    error: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() {
    let port = env::var("LM_STUDIO_PORT").unwrap_or_else(|_| "1234".to_string());
    let model = env::var("LM_MODEL").unwrap_or_else(|_| "google/gemma-4-e4b".to_string());

    let client = Client::new();
    let url = format!("http://localhost:{}/v1/chat/completions", port);

    let mut stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = buffer.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let response = handle_request(&client, &url, &model, trimmed).await;
                if let Some(resp) = response {
                    println!("{}", resp);
                    io::stdout().flush().ok();
                }
            }
            Err(_) => break,
        }
    }
}

async fn handle_request(
    client: &Client,
    url: &str,
    model: &str,
    input: &str,
) -> Option<String> {
    let messages = vec![
        Message {
            role: "user".to_string(),
            content: input.to_string(),
        }
    ];

    let request_payload = serde_json::json!({
        "model": model,
        "messages": messages
    });

    match client.post(url).json(&request_payload).send().await {
        Ok(resp) => match resp.json::<LmStudioResponse>().await {
            Ok(lm_resp) => {
                if let Some(err) = &lm_resp.error {
                    let rpc_error = JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: None,
                        result: None,
                        error: Some(err.clone()),
                    };
                    return serde_json::to_string(&rpc_error).ok();
                }

                let content = lm_resp
                    .choices
                    .as_ref()
                    .and_then(|c| c.first())
                    .and_then(|c| c.message.as_ref())
                    .map(|m| m.content.clone())
                    .unwrap_or_else(|| "No response".to_string());

                let rpc_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: Some(serde_json::json!({ "content": content })),
                    error: None,
                };
                serde_json::to_string(&rpc_response).ok()
            }
            Err(e) => {
                let rpc_error = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(serde_json::json!({ "message": e.to_string() })),
                };
                serde_json::to_string(&rpc_error).ok()
            }
        },
        Err(e) => {
            let rpc_error = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: None,
                result: None,
                error: Some(serde_json::json!({ "message": e.to_string() })),
            };
            serde_json::to_string(&rpc_error).ok()
        }
    }
}