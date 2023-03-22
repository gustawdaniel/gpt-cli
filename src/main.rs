extern crate core;

mod cache;
mod decompose;
mod get_postprocess_action;
mod gpt3;
mod should_exit;

use inquire::Confirm;

use colored::*;
use std::env;

use crate::get_postprocess_action::{get_postprocess_action, PostprocessAction};
use crate::gpt3::Gpt3Message;
use crate::should_exit::{should_exit, ShouldExit};
use std::process::{Command, Stdio};
use tokio::runtime::Runtime;

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

                    child.wait().expect("Failed to wait for command");
                }
                Ok(false) => println!("That's too bad, I've heard great things about it."),
                Err(_) => println!("Error with questionnaire, try again later"),
            }
        }
        PostprocessAction::Copy => {
            #[cfg(not(target_env = "musl"))]
            {
                terminal_clipboard::set_string(answer_text).unwrap();
                assert_eq!(*answer_text, terminal_clipboard::get_string().unwrap());
                println!("Text '{answer_text}' was copied to your clipboard")
            }
            #[cfg(target_env = "musl")]
            {
                println!("{}", answer_text);
            }
        }
        PostprocessAction::Out => {
            println!("{}", answer_text);
        }
    }
}

fn exit_with_messages_if_required(should_exit: ShouldExit) {
    let ShouldExit {
        exit,
        messages,
        is_error,
    } = should_exit;

    if exit {
        for message in messages.iter() {
            if is_error {
                eprintln!("{}", message);
            } else {
                println!("{}", message)
            }
        }
        std::process::exit(1);
    }
}

async fn async_main() {
    let args: Vec<String> = env::args().skip(1).collect();
    exit_with_messages_if_required(should_exit(&args));

    let content = args.join(" ");
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = gpt3::Gpt::new(Some(false), None);
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

        match response {
            Err(error) => {
                let mut messages = vec![error.red()];
                if error == *"Error: GPT3_API_KEY environment variable is not defined." {
                    messages.push(
                        "Please set the GPT3_API_KEY environment variable to your OpenAI API key."
                            .normal(),
                    );
                }
                exit_with_messages_if_required(ShouldExit {
                    is_error: true,
                    exit: true,
                    messages,
                });
            }
            Ok(data) => {
                let choice = data.choices.first().expect("No choice in response");
                let answer_text = &choice.message.content;

                postprocess(answer_text);
            }
        }
    });
}

fn main() {
    futures::executor::block_on(async_main());
}
