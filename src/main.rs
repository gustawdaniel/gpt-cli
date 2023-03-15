mod cache;
mod decompose;
mod gpt3;

use inquire::Confirm;

use colored::*;
use std::env;

use crate::gpt3::Gpt3Message;
use std::process::{Command, Stdio};
use tokio::runtime::Runtime;

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
            } else {
            }
            std::process::exit(1);
        }

        let data = response.expect("Unhandled error");

        let choice = data.choices.first().expect("No choice in response");
        let command = &choice.message.content;

        let ans = Confirm::new(&format!("Execute.:\n\n{}\n\n", command.green()))
            .with_default(true)
            .with_help_message("Pressing enter you confirm execution of this command")
            .prompt();

        match ans {
            Ok(true) => {
                let (command_name, command_args) = decompose::decompose(command);

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
    });
}

fn main() {
    futures::executor::block_on(async_main());
}
