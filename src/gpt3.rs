use crate::cache::Cache;
use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::thread;
use std::time::Duration;

pub(crate) struct Gpt {
    debug: bool,
    api_key: String,
}

impl Gpt {
    pub(crate) fn new(debug: Option<bool>) -> Self {
        let api_key = match std::env::var("GPT3_API_KEY") {
            Ok(val) => val,
            Err(_) => String::from(""),
        };

        Self {
            api_key,
            debug: debug.unwrap_or(false),
        }
    }

    pub(crate) fn get_system_prompt() -> String {
        match std::env::var("GPT_SYSTEM_PROMPT") {
            Ok(val) => val,
            Err(_) => String::from("Imagine you are linux terminal commands selector. I will describe task and you will respond only using linux command, without description, without explanation.")
        }
    }

    fn check_api_key(&self) -> Result<(), String> {
        if self.api_key.len().gt(&0) {
            Ok(())
        } else {
            Err(String::from(
                "Error: GPT3_API_KEY environment variable is not defined.",
            ))
        }
    }

    #[async_recursion]
    pub(crate) async fn ask(&self, messages: Vec<Gpt3Message>) -> Result<Gpt3Response, String> {
        if self.debug {
            let response = Gpt3Response {
                id: "chatcmpl-6taJ9NwJAFdKNafz0Y49j5ga0jFiF".to_string(),
                object: "chat.completion".to_string(),
                created: 1678705627,
                model: "gpt-3.5-turbo-0301".to_string(),
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

        if let Some(cached_data) = cache.get(&key) {
            let response: Gpt3Response = serde_json::from_str(&cached_data).unwrap();
            return Ok(response);
        }

        let data = json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        });

        let response = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
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
                    cache.set(&key, &response_str);
                }
                Some(_) => {
                    println!("{:?}", json.choices[0]);
                }
            }

            Ok(json)
        } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            // Get the error response body as a string
            let error_body = response.text().await.map_err(|e| format!("{e}"))?;
            // Parse the JSON error response
            let error_json = serde_json::from_str::<serde_json::Value>(&error_body)
                .map_err(|e| format!("{e}"))?;
            // Extract the error message and recommended wait time
            // let error_message = error_json["message"].as_str().unwrap_or_default();
            let seconds_to_wait = error_json["seconds_to_wait"].as_u64().unwrap_or_default();
            // Print the error message
            // Err(format!("Request failed with status code: {}\nError message: {}\nRecommended wait time: {} seconds", status, error_message, seconds_to_wait))
            thread::sleep(Duration::from_secs(seconds_to_wait));

            self.ask(messages).await
        } else {
            // Get the error response body as a string
            let error_body = response.text().await.map_err(|e| format!("{e}"))?;
            // Print the error message
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
            "Imagine you are linux terminal commands selector. I will describe task and you will respond only using linux command, without description, without explanation."
        );
    }

    #[test]
    fn test_ask_with_env_var_and_debug_true() {
        std::env::set_var("GPT3_API_KEY", "test_key");
        let gpt = Gpt::new(Some(true));
        let messages = vec![Gpt3Message {
            content: "hello".to_string(),
            role: "user".to_string(),
        }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_ok());
        // Check that the response is correct
        let response = result.unwrap();
        assert_eq!(response.model, "gpt-3.5-turbo-0301");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].finish_reason, Some("stop".to_string()));
    }

    #[test]
    fn test_ask_without_env_var() {
        std::env::remove_var("GPT3_API_KEY");
        let gpt = Gpt::new(Some(false));
        let messages = vec![Gpt3Message {
            content: "hello".to_string(),
            role: "user".to_string(),
        }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_err());
        // Check that the error message is correct
        let error = result.unwrap_err();
        assert!(error.contains("Error: GPT3_API_KEY environment variable is not defined."));
    }
}
