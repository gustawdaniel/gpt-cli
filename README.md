[![Test](https://github.com/gustawdaniel/gpt-cli/actions/workflows/test.yml/badge.svg)](https://github.com/gustawdaniel/gpt-cli/actions/workflows/test.yml)
[![Release](https://github.com/gustawdaniel/gpt-cli/actions/workflows/release.yml/badge.svg)](https://github.com/gustawdaniel/gpt-cli/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/gustawdaniel/gpt-cli/branch/main/graph/badge.svg?token=KB4F1JF9W6)](https://codecov.io/gh/gustawdaniel/gpt-cli)
![Crates.io](https://img.shields.io/crates/d/gpt-cli?label=Crates.io%20downloads)
![Docker Pulls](https://img.shields.io/docker/pulls/gustawdaniel/gpt-cli)
![Crates.io](https://img.shields.io/crates/v/gpt-cli)
![GitHub](https://img.shields.io/github/license/gustawdaniel/gpt-cli)

# Gpt Cli

Linux terminal GPT3 integration with killer prompt that skip descriptions and other human-readable bullshit. It shows
you commands that can be executed by `ENTER`.

## Example:

You typing:

```
p show me my graphic cards
```

You will see:

```
lspci -k | grep -A 2 -E "(VGA|3D)"
```

After `ENTER` you will see

```
00:02.0 VGA compatible controller: Intel Corporation Alder Lake-P Integrated Graphics Controller (rev 0c)
        Subsystem: CLEVO/KAPOK Computer Device 65f5
        Kernel driver in use: i915
--
01:00.0 VGA compatible controller: NVIDIA Corporation GA106M [GeForce RTX 3060 Mobile / Max-Q] (rev a1)
        Subsystem: CLEVO/KAPOK Computer Device 67f5
        Kernel driver in use: nvidia
```

## More powerful every day

From v0.0.8 it supports context overriding. Default system context is

> Imagine you are linux terminal command selector. I will describe task, and you will respond only using linux command,
> without description, without explanation.

Default postprocess mode is `confirm`. It shows answer and asking if it should be executed.

But you can use it in other use-cases. To translate texts:

```
GPT_SYSTEM_PROMPT="I am translator from polish to english. I need to translate this text." GPT_POST=copy p Witaj świecie
```

You can redirect it to file set these environment variables as permanent.

```
export GPT_SYSTEM_PROMPT="I am translator from polish to english. I need to translate this text."; export GPT_POST=out;
```

and then translate using:

```
p "$(cat polish.txt)" > english.txt
```

To back do default

```
unset GPT_SYSTEM_PROMPT; unset GPT_POST
```

Possible values:

- `GPT_SYSTEM_PROMPT` - any string that will explain gpt3 how to behave.
- `GPT_POST`
    - confirm - default, will ask if execute output in terminal
    - copy - will copy your answer to terminal clipboard
    - out - will print answer on standard output - usefully for further processing

## Installation

There are few options

### Shell

```
wget -qO- https://raw.githubusercontent.com/gustawdaniel/gpt-cli/main/install.sh | bash
```

it will save `gpt-cli` and alias `p` in `/usr/local/bin` so this is why it require sudo.

### Cargo

```
cargo install gpt-cli
ln -s ~/.cargo/bin/gpt-cli ~/.cargo/bin/p
```

### Docker

```
alias p="docker run -v ~/.gpt-cache.json:/.gpt-cache.json -e GPT3_API_KEY=${GPT3_API_KEY} gustawdaniel/gpt-cli"
```

In Docker, you can't use flag `GPT_POST` and it is automatically set as `out`. It means that you can't confirm command
by `ENTER` and commands will not be copied to your clipboard.

## Complication from source

```
git clone https://github.com/gustawdaniel/gpt-cli && cd gpt-cli 
cargo build --release
sudo cp ./target/release/gpt-cli /usr/local/bin/p
```

## Config

Copy your `GPT3_API_KEY` to env variable. Your `.profile`, `.bashrc`, or `.zshrc` file.

```bash
export GPT3_API_KEY=sk-xxx
```

You'd need to enter your own OpenAI API key Here's how you can get one

1. Go to https://openai.com/api/login
2. Create an account or log into your existing account
3. Go to https://platform.openai.com/account/api-keys or

![](https://user-images.githubusercontent.com/36589645/202097820-dc6905e6-4514-413b-980f-169c35ffef9a.png)

Price: `$0.002 per 1,000 tokens`. Single command is about 50 tokens. So in price 1USD you have about `10.000` commands.
Tools with model before `gpt-3.5-turbo` costs 10 times more.

## Usage

| what you typing in terminal                                                        | answers you can execute by "ENTER"                                            |
|------------------------------------------------------------------------------------|-------------------------------------------------------------------------------|
| show me graphic card                                                               | lspci -k &#124; grep -A 2 -E "(VGA&#124;3D)"                                  |
| jq command that join package name and version by dash                              | jq '.name + "-" + .version'                                                   |
| three processes with highest ram usage                                             | ps aux --sort=-%mem &#124; head -n 4                                          |
| make backup of mysql db called docs                                                | mysqldump docs > docs_backup.sql                                              |
| setup jest configured for typescript                                               | npm install --save-dev jest @types/jest ts-jest                               |
| generate ed keys                                                                   | openssl genpkey -algorithm ed25519 -out privatekey.key                        |
| show me content of Cargo.toml encoded as base64                                    | base64 Cargo.toml                                                             |
| show me content of Cargo.toml encoded as base64 in single line                     | cat Cargo.toml &#124; base64 -w 0                                             |
| show timer that will update every second                                           | watch -n 1 date +%T                                                           |
| range from 10 to 15                                                                | seq 10 15                                                                     |
| replace all lines starting from "CFG_" to starting from "CONFIG_" in file env.conf | sed -i 's/^CFG_/CONFIG_/g' env.conf                                           |
| write one liner to detect emails in file                                           | grep -Eio '\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z&#124;a-z]{2,}\b' filename |
| cron line to execute /bin/task every monday at 7pm                                 | 0 19 * * 1 /bin/task                                                          |
| run rusts tests one by one                                                         | cargo test -- --test-threads=1                                                |
| who i am and am i still needed                                                     | whoami and yes, you are still needed. <-- hahah it is authentic gpt3 answer   |

## Features

- [x] Interactive commands
- [x] Colors
- [x] Real time stream
- [x] Overriding system context (`GPT_SYSTEM_PROMPT` env)
- [x] Confirm, Copy and Standard Output modes (`GPT_POST` env)
- [ ] Easy to install (in progress)
    - [x] compilation from source
    - [x] install by bash like nvm
    - [x] docker
    - [ ] snap
    - [x] aur
    - [ ] apt
    - [ ] dnf

## Exceptions

If commands contains `export` or `$` it can't be correctly passed from child process to parent.
So there is fallback applied and these commands are copied if you wanted to execute them by confirmation.

Examples:

```
p change terminal language to english
Text 'export LANG=en_US.UTF-8' was copied to your clipboard

p show my current shell
Text 'echo $SHELL' was copied to your clipboard
```

## Constrains

Ofc GPT3 does not have sense of humor... so

```bash
p say mooo as cow that have colors of rainbow
```

will not work. Correct answer is

```bash
cowsay mooo | lolcat
```

and for

```bash
p show my train in terminal
```

answer is

```bash
Sorry, I do not understand. Can you please provide more details about what you want me to do?
```

instead

```bash
sl
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=gustawdaniel/gpt-cli&type=Date)](https://star-history.com/#gustawdaniel/gpt-cli&Date)

## Alternatives

Stars was updated 14-03-2023

| This project    |                                                               |
|-----------------|---------------------------------------------------------------|
| Language        | rust                                                          |
| Easy to install | no (wip)                                                      |
| Streaming       | yes                                                           |
| Stars           | 4                                                             |
| Release         | 13-03-2023                                                    |
| Last update     | 14-03-2023                                                    |
| Engine          | gpt-3.5-turbo                                                 |
| Goal            | save time for cli commands typing if you do not remember them |

| gpt3-cli        | https://github.com/CrazyPython/gpt3-cli                 |
|-----------------|---------------------------------------------------------|
| Language        | python                                                  |
| Easy to install | medium                                                  |
| Streaming       | yes                                                     |
| Stars           | 50                                                      |
| Release         | 23-03-2021                                              |
| Last update     | 22-04-2021                                              |
| Engine          | all before gpt-3.5-turbo                                |
| Goal            | A lightweight command-line interface to OpenAI's GPT-3. |

| ai-cli          | https://github.com/abhagsain/ai-cli                             |
|-----------------|-----------------------------------------------------------------|
| Language        | typescript                                                      |
| Easy to install | yes                                                             |
| Streaming       | no                                                              |
| Stars           | 935                                                             |
| Release         | 15-11-2022                                                      |
| Last update     | 09-12-2022                                                      |
| Engine          | all before gpt-3.5-turbo                                        |
| Goal            | Get answers for CLI commands from GPT3 right from your terminal |

| heygpt          | https://github.com/fuyufjh/heygpt               |
|-----------------|-------------------------------------------------|
| Language        | rust                                            |
| Easy to install | yes                                             |
| Streaming       | yes                                             |
| Stars           | 40                                              |
| Release         | 06-03-2023                                      |
| Last update     | 12-03-2023                                      |
| Engine          | gpt-3.5-turbo                                   |
| Goal            | A simple common-line interface for ChatGPT API. |

| caos            | https://github.com/dabumana/caos                                    |
|-----------------|---------------------------------------------------------------------|
| Language        | go                                                                  |
| Easy to install | no                                                                  |
| Streaming       | no                                                                  |
| Stars           | 5                                                                   |
| Release         | 22-01-2023                                                          |
| Last update     | 13-03-2023                                                          |
| Engine          | all before gpt-3.5-turbo                                            |
| Goal            | advanced, configurable conversational assistant for openai services |

| gptsh           | https://github.com/shorwood/gptsh                                     |
|-----------------|-----------------------------------------------------------------------|
| Language        | javascript                                                            |
| Easy to install | yes                                                                   |
| Streaming       | no                                                                    |
| Stars           | 99                                                                    |
| Release         | 27-12-2020                                                            |
| Last update     | 18-01-2022                                                            |
| Engine          | all before gpt-3.5-turbo                                              |
| Goal            | translate natural language questions and requests into shell commands |

| rusty           | https://github.com/zahidkhawaja/rusty |
|-----------------|---------------------------------------|
| Language        | rust                                  |
| Easy to install | no                                    |
| Streaming       | no                                    |
| Stars           | 272                                   |
| Release         | 05-09-2022                            |
| Last update     | 07-02-2023                            |
| Engine          | text-davinci-003                      |
| Goal            | help you remember bash commands       |

| cgpt            | https://github.com/MagicCube/cli-gpt                   |
|-----------------|--------------------------------------------------------|
| Language        | typescript                                             |
| Easy to install | yes                                                    |
| Streaming       | no                                                     |
| Stars           | 18                                                     |
| Release         | 07-03-2023                                             |
| Last update     | 15-03-2023                                             |
| Engine          | gpt-3.5-turbo                                          |
| Goal            | Translate human language to command line using ChatGPT |

| linux-command-gpt | https://github.com/asrul10/linux-command-gpt                      |
|-------------------|-------------------------------------------------------------------|
| Language          | go                                                                |
| Easy to install   | no                                                                |
| Streaming         | yes                                                               |
| Stars             | 54                                                                |
| Release           | 12-03-2023                                                        |
| Last update       | 19-03-2023                                                        |
| Engine            | gpt-3.5-turbo                                                     |
| Goal              | Get Linux commands in natural language with the power of ChatGPT. |

## GNU vs MUSL releases

During compilation, you can use static linking (musl) or dynamic (gnu). To use `terminal-clipboard` there is required
need dynamic linking, but it works only on typical linuxes that uses libc. To make docker image small (12 MB) there is
provided `musl` version.

So to be able to use all features (support for GPT_POST=copy), I recommend to use standard `gnu` but if you need docker
or run it on alpine then use `musl`.

## Support

I'm looking for challenging, remote job with rust + typescript + advanced math, so if you appreciate this project, you
can share it, leave star, and recommend me earning employment commission.
