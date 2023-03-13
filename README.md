[![Test](https://github.com/gustawdaniel/gpt-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/gustawdaniel/gpt-cli/actions/workflows/rust.yml)
[![Release](https://github.com/gustawdaniel/gpt-cli/actions/workflows/release.yml/badge.svg)](https://github.com/gustawdaniel/gpt-cli/actions/workflows/release.yml)

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

## Installation

```
cargo build --release
sudo cp ./target/release/gpt-cli /usr/local/bin/p
```

## Config

Copy your `GPT3_API_KEY` to env variable. Your `.profile`, `.bashrc`, or `.zshrc` file.

```bash
export GPT3_API_KEY=sk-xxx
```

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

## Support

I'm looking for challenging, remote job with rust + typescript + advanced math, so if you appreciate this project, you
can share it and recommend me and earn employment commission.