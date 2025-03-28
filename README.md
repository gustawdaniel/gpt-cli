![GitHub](https://img.shields.io/github/license/openfinesse/gpt-cli)

# AI CLI

Intergration that allows you to use natural language in the terminal.

## Usage:

```bash
user@system:~$ p show me my graphic cards
? Execute.:

lspci | grep -i vga

 (Y/n)  
[Pressing enter you confirm execution of this command]
```

After `ENTER` you will see

```bash
00:02.0 VGA compatible controller: Intel Corporation CometLake-H GT2 [UHD Graphics] (rev 05)
01:00.0 VGA compatible controller: NVIDIA Corporation TU117M [GeForce GTX 1650 Ti Mobile] (rev a1)
```

## Customization

### Context and output mode

Default system context is:

> You are a linux terminal command generator. I will describe a task and you will respond with linux command,
> do not include any description, explanation or any extrenous syntax.

Default postprocess mode is `confirm`. It presents an answer and asks for confirmation before execution.

But you can use it for other use-cases. For example, to translate texts:

```
GPT_SYSTEM_PROMPT="I am a translator from polish to english. I need to translate this text." GPT_POST=copy p Witaj świecie
```

Set your custom system prompt and postprocess mode by setting environment variables (.profile, .bashrc, .zshrc, etc.):

```
export GPT_SYSTEM_PROMPT="I am translator from polish to english. I need to translate this text."; export GPT_POST=out;
```

and then translate using:

```
p "$(cat polish.txt)" > english.txt
```

Revert by unsetting the environment variables:

```
unset GPT_SYSTEM_PROMPT; unset GPT_POST
```

Possible values:

- `GPT_SYSTEM_PROMPT` - any string that will explain gpt3 how to behave.
- `GPT_POST`
    - confirm - default, will ask if execute output in terminal
    - copy - will copy your answer to terminal clipboard
    - out - will print answer on standard output - usefully for further processing

### Model Selection

You can select your model by adding env variable `GPT_MODEL` and can also use non OpenAI models by setting `OPENAI_BASE_URL`.

```bash
export GPT_MODEL=gpt-4o
```
OR

```bash
export OPENAI_BASE_URL=https://openrouter.ai/api/v1
export GPT_MODEL=anthropic/claude-3.7-sonnet
```

## Installation

There are few options

### Shell

You need: `wget` and `sudo`.

```
wget -qO- https://raw.githubusercontent.com/gustawdaniel/gpt-cli/main/install.sh | bash
```

it will save `gpt-cli` and alias `p` in `/usr/local/bin`

### Cargo

```
cargo install gpt-cli
ln -s ~/.cargo/bin/gpt-cli ~/.cargo/bin/p
```

### Docker

```
alias p="docker run -v ~/.gpt-cache.json:/.gpt-cache.json -e OPENAPI_API_KEY=${OPENAPI_API_KEY} gustawdaniel/gpt-cli"
```

In Docker, you can't use flag `GPT_POST` and it is automatically set as `out`. It means that you can't confirm command
by `ENTER` and commands will not be copied to your clipboard.

## Compilation from source

```
git clone https://github.com/openfinesse/gpt-cli && cd gpt-cli 
cargo build --release
sudo cp ./target/release/gpt-cli /usr/local/bin/p
```

## Config

Make sure you have `OPENAI_API_KEY` set in your environment variables. IE: Your `.profile`, `.bashrc`, or `.zshrc` file.

```bash
export OPENAI_API_KEY=sk-xxx
```

You'd need to enter your own OpenAI API key. Here's how you can get one:

1. Go to https://openai.com/api/login
2. Create an account or log into your existing account
3. Go to https://platform.openai.com/account/api-keys or


## Examples

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
- [x] Add support for non-OpenAI models

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

## GNU vs MUSL releases

During compilation, you can use static linking (musl) or dynamic (gnu). To use `terminal-clipboard` there is required
need dynamic linking, but it works only on typical linuxes that uses libc. To make docker image small (12 MB) there is
provided `musl` version.

So to be able to use all features (support for GPT_POST=copy), I recommend to use standard `gnu` but if you need docker
or run it on alpine then use `musl`.

## Credit

- [Daniel Gustaw](https://github.com/gustawdaniel) - original author