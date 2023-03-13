use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::cache::Cache;

pub(crate) struct GPT {
    debug: bool,
    api_key: String,
}


impl GPT {
    pub(crate) fn new(debug: Option<bool>) -> Self {
        let api_key = match std::env::var("GPT3_API_KEY") {
            Ok(val) => val,
            Err(_) => String::from("")
        };

        Self {
            api_key,
            debug: debug.unwrap_or(false),
        }
    }

    fn check_api_key(&self) -> Result<(), String> {
        if self.api_key.len().gt(&0) {
            Ok(())
        } else {
            Err(String::from("Error: GPT3_API_KEY environment variable is not defined."))
        }
    }

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
                    finish_reason: "stop".to_string(),
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
            .map_err(|e| format!("{}", e))?
            .json::<Gpt3Response>()
            .await
            .map_err(|e| format!("{}", e))?;

        if response.choices[0].finish_reason == "stop" {
            let response_str = serde_json::to_string(&response).unwrap();
            cache.set(&key, &response_str);
        } else {
            println!("{:?}", response.choices[0]);
        }

        Ok(response)
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
    finish_reason: String,
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
    fn test_ask_with_env_var_and_debug_true() {
        std::env::set_var("GPT3_API_KEY", "test_key");
        let gpt = GPT::new(Some(true));
        let messages = vec![Gpt3Message { content: "hello".to_string(), role: "user".to_string() }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_ok());
        // Check that the response is correct
        let response = result.unwrap();
        assert_eq!(response.model, "gpt-3.5-turbo-0301");
        assert_eq!(response.choices.len(), 1);
        assert_eq!(response.choices[0].finish_reason, "stop");
    }

    #[test]
    fn test_ask_without_env_var() {
        std::env::remove_var("GPT3_API_KEY");
        let gpt = GPT::new(Some(false));
        let messages = vec![Gpt3Message { content: "hello".to_string(), role: "user".to_string() }];
        let result = futures::executor::block_on(gpt.ask(messages));
        assert!(result.is_err());
        // Check that the error message is correct
        let error = result.unwrap_err();
        assert!(error.contains("Error: GPT3_API_KEY environment variable is not defined."));
    }
}