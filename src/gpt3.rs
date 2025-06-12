use crate::cache::Cache;
use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::thread;
use std::time::Duration;

pub(crate) struct Gpt {
    debug: bool,
    api_key: String,
    openapi_host: String,
}

impl Gpt {
    const OPEN_AI_HOST: &'static str = "https://api.openai.com";

    pub(crate) fn new(debug: Option<bool>, openapi_host: Option<&str>) -> Self {
        let api_key = std::env::var("OPENAI_API_KEY")
            .unwrap_or_else(|_| String::new());

        let openapi_host = std::env::var("OPENAI_BASE_URL")
            .ok()
            .unwrap_or_else(|| String::from(openapi_host.unwrap_or(Gpt::OPEN_AI_HOST)));

        Self {
            api_key,
            debug: debug.unwrap_or(false),
            openapi_host,
        }
    }

    pub fn is_open_ai(&self) -> bool {
        self.openapi_host == Gpt::OPEN_AI_HOST
    }

    pub(crate) fn get_system_prompt() -> String {
        match std::env::var("GPT_SYSTEM_PROMPT") {
            Ok(val) => val,
            Err(_) => String::from("You are a linux terminal command generator. I will describe a task and you will respond with linux command, do not include any description, explanation or any extrenous syntax.")
        }
    }

    fn check_api_key(&self) -> Result<(), String> {
        if self.api_key.len().gt(&0) {
            Ok(())
        } else {
            Err(String::from(
                "Error: OPENAI_API_KEY environment variable is not defined.",
            ))
        }
    }

    #[async_recursion]
    pub(crate) async fn ask(&self, messages: Vec<Gpt3Message>) -> Result<Gpt3Response, String> {
        let base_url = format!("{}/v1/chat/completions", self.openapi_host);

        if self.debug {
            let response = Gpt3Response {
                id: "chatcmpl-6taJ9NwJAFdKNafz0Y49j5ga0jFiF".to_string(),
                object: "chat.completion".to_string(),
                created: 1678705627,
                model: "gpt-4o".to_string(),
                usage: Usage {
                    prompt_tokens: 45,
                    completion_tokens: 3,
                    total_tokens: 48,
                },
                choices: vec![Choice {
                    message: Message {
                        role: "assistant".to_string(),
                        content: "npx ncu -i".to_string(),
                    },
                    finish_reason: Some("stop".to_string()),
                    index: 0,
                }],
            };
            return Ok(response);
        }

        self.check_api_key()?;

        let mut cache = Cache::new(None);
        let key = serde_json::to_string(&messages).unwrap();

        if self.is_open_ai() {
            if let Some(cached_data) = cache.get(&key) {
                let response: Gpt3Response = serde_json::from_str(&cached_data).unwrap();
                return Ok(response);
            }
        }

        let model = std::env::var("GPT_MODEL").unwrap_or_else(|_| String::from("gpt-4o"));

        let data = json!({
            "model": model,
            "messages": messages
        });

        let response = reqwest::Client::new()
            .post(base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&data)
            .send()
            .await
            .map_err(|e| format!("{e}"))?;

        let status = response.status();

        if status.is_success() {
            // Parse the response body
            let json = response
                .json::<Gpt3Response>()
                .await
                .map_err(|e| format!("{e}"))?;

            // Use the parsed data
            match json.choices[0].finish_reason.as_deref() {
                Some("stop") | Some("") | None => {
                    let response_str = serde_json::to_string(&json).unwrap();
                    if self.is_open_ai() {
                        cache.set(&key, &response_str);
                    }
                }
                Some(_) => {
                    println!("{:?}", json.choices[0]);
                }
            }

            Ok(json)
        } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            let error_body = response.text().await.map_err(|e| format!("{e}"))?;
            let error_json = serde_json::from_str::<serde_json::Value>(&error_body)
                .map_err(|e| format!("{e}"))?;
            let seconds_to_wait = error_json["seconds_to_wait"].as_u64().unwrap_or_default();
            thread::sleep(Duration::from_secs(seconds_to_wait));

            self.ask(messages).await
        } else {
            let error_body = response.text().await.map_err(|e| format!("{e}"))?;
            Err(format!(
                "Request failed with status code: {}\nError response body: {}",
                status, error_body
            ))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Gpt3Message {
    pub(crate) role: String,
    pub(crate) content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gpt3Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    finish_reason: Option<String>,
    index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_prompt_with_env_var() {
        std::env::set_var("GPT_SYSTEM_PROMPT", "Custom prompt");
        let prompt = Gpt::get_system_prompt();
        assert_eq!(prompt, "Custom prompt");
    }

    #[test]
    fn test_get_system_prompt_without_env_var() {
        std::env::remove_var("GPT_SYSTEM_PROMPT");
        let prompt = Gpt::get_system_prompt();
        assert_eq!(
            prompt,
            "You are a linux terminal command generator. I will describe a task and you will respond with linux command, do not include any description, explanation or any extrenous syntax."
        );
    }

    #[test]
    fn test_ask_with_env_var_and_debug_true() {
        std::env::set_var("OPENAI_API_KEY", "test_key");
        let gpt = Gpt::new(Some(true), None);
        let messages = vec![Gpt3Message {
            content: "hello".to_string(),
            role: "user".to_string(),
        }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_ok());
        // Check that the response is correct
        let response = result.unwrap();
        assert_eq!(response.model, "gpt-4o");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].finish_reason, Some("stop".to_string()));
    }

    #[test]
    fn test_ask_without_env_var() {
        std::env::remove_var("OPENAI_API_KEY");
        let gpt = Gpt::new(Some(false), None);
        let messages = vec![Gpt3Message {
            content: "hello".to_string(),
            role: "user".to_string(),
        }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_err());
        // Check that the error message is correct
        let error = result.unwrap_err();
        assert!(error.contains("Error: OPENAI_API_KEY environment variable is not defined."));
    }

    #[tokio::test]
    async fn test_http_mock() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.path("/hi");
            then.status(200);
        });

        let response = reqwest::get(format!("{}/hi", server.url("")))
            .await
            .unwrap();

        assert_eq!(response.status(), 200);
        mock.assert();
    }

    #[tokio::test]
    async fn test_ask_success() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v1/chat/completions");
            then.status(200).json_body_obj(&json!({
                "id": "testid",
                "object": "chat.completion",
                "created": 1678705627,
                "model": "gpt-4o",
                "usage": { "prompt_tokens": 45, "completion_tokens": 3, "total_tokens": 48 },
                "choices": [
                    {
                        "message": { "role": "assistant", "content": "npx ncu -i" },
                        "finish_reason": "stop",
                        "index": 0
                    }
                ]
            }));
        });

        std::env::set_var("OPENAI_API_KEY", "test_key");
        let gpt = Gpt::new(Some(false), Some(&server.url("")));
        let messages = vec![
            Gpt3Message {
                role: "system".to_string(),
                content: Gpt::get_system_prompt(),
            },
            Gpt3Message {
                role: "user".to_string(),
                content: "Update all npm packages to the latest version.".to_string(),
            },
        ];

        // Call
        let response = gpt.ask(messages).await.unwrap();

        // Assert
        assert_eq!(
            response.choices[0].message.content,
            "npx ncu -i".to_string()
        );
        mock.assert();
    }

    #[tokio::test]
    async fn test_ask_fail() {
        let server = httpmock::MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v1/chat/completions");
            then.status(500).json_body_obj(&json!({
                "status": "error",
                "message": "unexpected error"
            }));
        });

        std::env::set_var("OPENAI_API_KEY", "test_key");
        let gpt = Gpt::new(Some(false), Some(&server.url("")));
        let messages = vec![
            Gpt3Message {
                role: "system".to_string(),
                content: Gpt::get_system_prompt(),
            },
            Gpt3Message {
                role: "user".to_string(),
                content: "Update all npm packages to the latest version.".to_string(),
            },
        ];

        // Call
        match gpt.ask(messages).await {
            Ok(_) => assert!(false, "Error was expected from mock."),
            Err(error_message) => {
                // Assert
                assert_eq!(
                    error_message,
                    "Request failed with status code: 500 Internal Server Error\nError response body: {\"message\":\"unexpected error\",\"status\":\"error\"}"
                );
                mock.assert();
            }
        }
    }
}
