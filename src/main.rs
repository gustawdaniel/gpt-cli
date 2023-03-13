mod decompose;
mod cache;
mod gpt3;
use inquire::{Confirm};

use colored::*;
use std::env;

use std::process::{Command, Stdio};
// use std::thread::Builder;
use crate::gpt3::Gpt3Message;
use tokio::runtime::Runtime;

async fn async_main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len().eq(&0) {
        eprintln!("{}", "Please add description, which command you want to execute.".red());
        eprintln!("eg.: cargo run -- show calendar");
        std::process::exit(1);
    }

    let content = args.join(" ");

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = gpt3::GPT::new(Some(false));
        let response = client.ask(vec![
            Gpt3Message {
                content: String::from("Imagine you are linux terminal commands selector. I will describe task and you will respond only using linux command, without description, without explanation."),
                role: String::from("system"),
            },
            Gpt3Message {
                role: String::from("user"),
                content,
            },
        ]).await;


        if response.is_err() {
            let error = response.unwrap_err();
            eprintln!("{}", error.red());
            eprintln!("Please set the GPT3_API_KEY environment variable to your OpenAI API key.");
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

                // let mut child = Command::new(command_name).args(command_args)
                let mut child = Command::new(command_name).args(command_args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Failed to execute command");

                let status = child.wait().expect("Failed to wait for command");

                // println!("Command exited with status: {}", status);
            }
            Ok(false) => println!("That's too bad, I've heard great things about it."),
            Err(_) => println!("Error with questionnaire, try again later"),
        }


    });
}

fn main() {
    futures::executor::block_on(async_main());
}