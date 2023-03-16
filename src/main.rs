mod cache;
mod decompose;
mod gpt3;

use inquire::Confirm;

use colored::*;
use std::env;

use crate::gpt3::Gpt3Message;
use std::process::{Command, Stdio};
use tokio::runtime::Runtime;

#[derive(Debug, PartialEq)]
enum PostprocessAction {
    // default
    Confirm,
    Copy,
    Out,
}

fn get_postprocess_action(answer_text: &str) -> PostprocessAction {
    let action_by_env = match env::var("GPT_POST") {
        Ok(val) => match val.as_str() {
            "confirm" => PostprocessAction::Confirm,
            "copy" => PostprocessAction::Copy,
            "out" => PostprocessAction::Out,
            _ => PostprocessAction::Confirm,
        },
        Err(_) => PostprocessAction::Confirm,
    };

    if (answer_text.contains('$') || answer_text.starts_with("export"))
        && action_by_env == PostprocessAction::Confirm
    {
        return PostprocessAction::Copy;
    }

    action_by_env
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_postprocess_action_confirm() {
        env::remove_var("GPT_POST");
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }

    #[test]
    fn test_get_postprocess_action_copy() {
        env::remove_var("GPT_POST");
        let answer = "This is an answer containing $variable.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_export() {
        env::remove_var("GPT_POST");
        let answer = "export MY_VARIABLE=value".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_env_confirm() {
        env::set_var("GPT_POST", "confirm");
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }

    #[test]
    fn test_get_postprocess_action_env_copy() {
        env::set_var("GPT_POST", "copy");
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Copy);
    }

    #[test]
    fn test_get_postprocess_action_env_out() {
        env::set_var("GPT_POST", "out");
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Out);
    }

    #[test]
    fn test_get_postprocess_action_env_invalid() {
        env::set_var("GPT_POST", "invalid");
        let answer = "This is a normal answer.".to_string();
        let action = get_postprocess_action(&answer);
        assert_eq!(action, PostprocessAction::Confirm);
    }
}

fn postprocess(answer_text: &String) {
    let action = get_postprocess_action(answer_text);

    match action {
        PostprocessAction::Confirm => {
            let ans = Confirm::new(&format!("Execute.:\n\n{}\n\n", answer_text.green()))
                .with_default(true)
                .with_help_message("Pressing enter you confirm execution of this command")
                .prompt();

            match ans {
                Ok(true) => {
                    let (command_name, command_args) = decompose::decompose(answer_text);

                    let mut child = Command::new(command_name)
                        .args(command_args)
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .expect("Failed to execute command");

                    let _ = child.wait().expect("Failed to wait for command");

                    // println!("Command exited with status: {}", _);
                }
                Ok(false) => println!("That's too bad, I've heard great things about it."),
                Err(_) => println!("Error with questionnaire, try again later"),
            }
        }
        PostprocessAction::Copy => {
            terminal_clipboard::set_string(answer_text).unwrap();
            assert_eq!(*answer_text, terminal_clipboard::get_string().unwrap());
            println!("Text '{answer_text}' was copied to your clipboard")
        }
        PostprocessAction::Out => {
            println!("{}", answer_text);
        }
    }
}

async fn async_main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len().eq(&0) {
        eprintln!(
            "{}",
            "Please add description, which command you want to execute.".red()
        );
        eprintln!("eg.: cargo run -- show calendar");
        std::process::exit(1);
    }

    let content = args.join(" ");
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = gpt3::Gpt::new(Some(false));
        let response = client
            .ask(vec![
                Gpt3Message {
                    content: gpt3::Gpt::get_system_prompt(),
                    role: String::from("system"),
                },
                Gpt3Message {
                    role: String::from("user"),
                    content,
                },
            ])
            .await;

        if let Err(error) = response {
            eprintln!("{}", error.red());
            if error == *"Error: GPT3_API_KEY environment variable is not defined." {
                eprintln!(
                    "Please set the GPT3_API_KEY environment variable to your OpenAI API key."
                );
            }
            std::process::exit(1);
        }

        let data = response.expect("Unhandled error");

        let choice = data.choices.first().expect("No choice in response");
        let answer_text = &choice.message.content;

        postprocess(answer_text);
    });
}

fn main() {
    futures::executor::block_on(async_main());
}
